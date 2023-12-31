use crate::common::*;
use crate::math::*;
use crate::physics::*;
use crate::render::*;
use crate::rf::Rf;
use rapier3d::prelude as rp;
use rapier3d::prelude::nalgebra as na;
use std::mem::replace;
use std::ptr::NonNull;
use tracing::debug;

/*
 * The following API functions are disabled for parent objects:
 * get_position_local, set_position_local, get_rotation_local, and
 * set_rotation_local.
 *
 * The following API functions are disabled for child objects:
 * apply_force, apply_torque, get_speed, get_velocity, get_angular_velocity,
 * set_position, and set_rotation.
 *
 * The following API functions only have an effect once the child is removed
 * from its parent: set_collidable, set_collision_group, set_collision_mask,
 * set_drag, set_friction, set_kinematic, set_restitution, and
 * set_sleep_threshold.
 *
 * The following API functions return information only about the current part
 * when the object is part of a compound: get_bounding_box_local,
 * get_bounding_box, and get_bounding_radius.
 *
 * The following API functions are only enabled for compound objects:
 * get_bounding_box_compound, get_bounding_box_local_compound,
 * and get_bounding_radius_compound.
 *
 * The local coordinate space of a child object is not scaled by the parent.
 * However, the position of the child will be multiplied by the parents scale.
 * Thus, the scale of the parent does not affect the size of the child and
 * local position is always 'relative to the parent'. A position of (1, 1, -1)
 * will always correspond to a point that will roughly coincide with the
 * right-top-front corner of the parents bounding box (assuming the vertices
 * of the mesh are contained in a cube that goes from (-1, -1, -1) to
 * (1, 1, 1)). When a parent is scaled the positions of children will be
 * multiplied in order to maintain the same relative position.
 */

// TODO: Implement Free semantics: Automatically frees all attached Triggers when called on a parent. Automatically frees all attached children and their Triggers when called on a parent. This function is O(M*N) for parents.
pub struct RigidBody {
    rigid_body: RigidBodyWrapper,
    collider: ColliderWrapper,
    parent: Option<NonNull<RigidBody>>, // Raw pointer to stable memory address of parent (as it's in a Box).
    children: Vec<rp::ColliderHandle>,

    // Fields to allow us to reconstruct the collision shape object.
    shape_type: CollisionShapeType,
    shape_scale: f32,

    collidable: bool,
    collision_group: rp::InteractionGroups,
}

impl RigidBody {
    pub fn new(shape: CollisionShape) -> Box<RigidBody> {
        let mut rigid_body = Box::new(RigidBody {
            rigid_body: RigidBodyWrapper::Removed(rp::RigidBodyBuilder::dynamic().build()),
            collider: ColliderWrapper::Removed(shape.collider),
            parent: None,
            children: vec![],
            shape_type: shape.shape,
            shape_scale: shape.scale,
            collidable: true,
            collision_group: rp::InteractionGroups::default(),
        });

        // The collider stores a reference to the handle for this rigid body
        // in its user data, which currently is just the stable raw pointer.
        rigid_body.collider.as_mut().user_data = RigidBody::encode_as_user_data(&rigid_body);

        rigid_body
    }

    pub(crate) fn add_to_world(
        &mut self,
        world: Rf<PhysicsWorld>,
    ) -> Option<(rp::RigidBodyHandle, rp::ColliderHandle)> {
        if self.collider.is_added() {
            return None;
        }

        // Add rigid body.
        let rb_handle = self.rigid_body.set_added(|rb| {
            let handle = world.as_mut().rigid_bodies.insert(rb);
            (handle, world.clone())
        });

        // Add collider.
        let collider_handle = self.collider.set_added(|collider| {
            let handle = {
                let w = &mut *world.as_mut();
                w.colliders
                    .insert_with_parent(collider, rb_handle, &mut w.rigid_bodies)
            };
            (handle, world)
        });

        Some((rb_handle, collider_handle))
    }

    pub(crate) fn remove_from_world(
        &mut self,
        impulse_joint_set: &mut rp::ImpulseJointSet,
        multibody_joint_set: &mut rp::MultibodyJointSet,
    ) -> Option<(rp::RigidBodyHandle, rp::ColliderHandle)> {
        if self.collider.is_removed() && self.rigid_body.is_removed() {
            return None;
        }

        // TODO: Remove children.

        debug!(
            "Removing rigid body {:?} with {:?} children",
            self as *mut _, self.children
        );

        let mut collider_handle = rp::ColliderHandle::invalid();
        let mut rigid_body_handle = rp::RigidBodyHandle::invalid();

        // Remove collider.
        self.collider.set_removed(|handle, world| {
            collider_handle = handle;
            let w = &mut *world.as_mut();
            w.colliders
                .remove(handle, &mut w.island_manager, &mut w.rigid_bodies, false)
                .unwrap()
        });

        // Remove rigid body.
        self.rigid_body.set_removed(|handle, world| {
            rigid_body_handle = handle;
            let w = &mut *world.as_mut();
            w.rigid_bodies
                .remove(
                    handle,
                    &mut w.island_manager,
                    &mut w.colliders,
                    impulse_joint_set,
                    multibody_joint_set,
                    false,
                )
                .unwrap()
        });

        Some((rigid_body_handle, collider_handle))
    }
    
    /// Links a RigidBody to a Rapier Collider, which we can later retrieve
    /// using linked_with_collider and linked_with_collider_mut.
    pub(crate) fn encode_as_user_data(rb: &Box<RigidBody>) -> u128 {
        &**rb as *const RigidBody as *mut RigidBody as u128
    }

    /// Retrieves a reference to the RigidBody linked to a Rapier Collider.
    ///
    /// The rest of the physics module guarantees that as long as a given
    /// collider exists, it's corresponding linked RigidBody exists as well.
    pub(crate) fn linked_with_collider(collider: &rp::Collider) -> Option<&'_ RigidBody> {
        if collider.user_data != 0 {
            let raw_ptr = collider.user_data as *const RigidBody;
            Some(unsafe { &*raw_ptr })
        } else {
            None
        }
    }

    /// Retrieves a mutable reference to the RigidBody linked to a Rapier Collider.
    ///
    /// The rest of the physics module guarantees that as long as a given
    /// collider exists, it's corresponding linked RigidBody exists as well.
    pub(crate) fn linked_with_collider_mut(collider: &rp::Collider) -> Option<&'_ mut RigidBody> {
        if collider.user_data != 0 {
            let raw_ptr = collider.user_data as *mut RigidBody;
            Some(unsafe { &mut *raw_ptr })
        } else {
            None
        }
    }

    /// Is this rigid body a child of another?
    pub fn is_child(&self) -> bool {
        self.parent.is_some()
    }

    /// Is this rigid body a parent?
    pub fn is_parent(&self) -> bool {
        !self.children.is_empty()
    }

    /// Returns the unscaled world matrix of this rigid body.
    fn get_world_matrix_unscaled(&self) -> Matrix {
        let global_transform = if let Some(parent) = self.get_parent() {
            let collider = self.collider.as_ref();
            let parent_rb = parent.rigid_body.as_ref();

            let default = rp::Isometry::default();
            let transform = collider.position_wrt_parent().unwrap_or(&default); //.expect(format!("child {:?} does not have a position_wrt parent {:?}", self as *const _, parent).as_str());
            let parent_transform = parent_rb.position();
            parent_transform * transform
        } else {
            self.rigid_body.as_ref().position().clone()
        };
        Matrix::from_rp(&global_transform)
    }

    // Returns a reference to the collider object.
    pub fn get_collider_ref(&self) -> RefOrBorrow<'_, rp::Collider> {
        self.collider.as_ref()
    }

    // Returns the rapier handle of the root rigid body if this rigid body is
    // added to the world or is attached to another, None otherwise.
    pub fn get_rigid_body_handle(&self) -> Option<rp::RigidBodyHandle> {
        if self.parent.is_some() {
            unsafe {
                self.parent
                    .as_ref()
                    .unwrap()
                    .as_ref()
                    .get_rigid_body_handle()
            }
        } else if self.rigid_body.is_added() {
            Some(*self.rigid_body.added_as_ref().unwrap().0)
        } else {
            None
        }
    }
}

#[luajit_ffi_gen::luajit_ffi(managed = true)]
impl RigidBody {
    #[bind(name = "CreateBox")]
    pub fn new_box() -> Box<RigidBody> {
        Self::new(CollisionShape::new_box(&Vec3::ONE))
    }

    #[bind(name = "CreateBoxFromMesh")]
    pub fn new_box_from_mesh(mesh: &mut Mesh) -> Box<RigidBody> {
        Self::new(CollisionShape::new_box_from_mesh(mesh))
    }

    #[bind(name = "CreateSphere")]
    pub fn new_sphere() -> Box<RigidBody> {
        Self::new(CollisionShape::new_sphere(1.0))
    }

    #[bind(name = "CreateSphereFromMesh")]
    pub fn new_sphere_from_mesh(mesh: &mut Mesh) -> Box<RigidBody> {
        Self::new(CollisionShape::new_sphere_from_mesh(mesh))
    }

    #[bind(name = "CreateHullFromMesh")]
    pub fn new_hull_from_mesh(mesh: &Mesh) -> Box<RigidBody> {
        Self::new(CollisionShape::new_hull_from_mesh(mesh))
    }

    /// Return a reference to the parent rigid body, that we can guarantee
    /// has a lifetime as long as self.
    #[bind(name = "GetParentBody")]
    pub fn get_parent(&self) -> Option<&mut RigidBody> {
        self.parent
            .as_ref()
            .map(|ptr| unsafe { &mut *ptr.as_ptr() })
    }

    pub fn apply_force(&mut self, force: &Vec3) {
        self.rigid_body.as_mut().add_force(force.to_na(), true);
    }

    pub fn apply_torque(&mut self, torque: &Vec3) {
        self.rigid_body.as_mut().add_torque(torque.to_na(), true);
    }

    /// Adds another rigid body as a child of this rigid body. This means that
    /// the child's position will be controlled by `self`.
    ///
    /// Only a single level of attachment is supported. Child objects do not
    /// affect the mass or inertia of the parent. Position is relative to the
    /// unscaled parent. i.e. it will be multiplied by the current scale. This
    /// function is O(1). Warning: if one object is attached to another and a
    /// third object happens to be between them this may trap the third object.
    /// The same issue may occur when spawning one compound inside another.
    ///
    /// This function assumes that `self` is not already a child.
    pub fn attach(&mut self, child: &mut RigidBody, pos: &Vec3, rot: &Quat) {
        if self as *mut _ == child as *mut _ {
            panic!("Cannot attach object to itself!");
        }

        if self.is_child() {
            panic!(
                "Recursive attachment is not supported. Parent is already attached to something."
            );
        }

        if self.collider.is_removed() && self.rigid_body.is_removed() {
            panic!("Parent has been removed from physics.");
        }

        if child.collider.is_added() && child.rigid_body.is_added() {
            panic!("Child has not been removed from physics.");
        }

        if child.is_child() {
            panic!("Child is already attached to a parent.");
        }

        debug!(
            "Attaching rigid body {:?} to {:?}",
            child as *mut _, self as *mut _
        );

        // Multiple colliders in Rapier can be attached to a single
        // rigid body, so there's no need to create a "compound shape"

        let (parent_handle, world) = self.rigid_body.added_as_ref().unwrap();

        // Compute the colliders relative position, scaled by the scale of the parent shape.
        let scaled_pos = *pos * self.shape_scale;

        // Add the child collider to the parent rigid body.
        let child_collider_handle = child.collider.set_added(|collider| {
            let handle = {
                // Lock the world as mutable.
                let w = &mut *world.as_mut();

                // Add the collider, and position it correctly.
                let handle =
                    w.colliders
                        .insert_with_parent(collider, *parent_handle, &mut w.rigid_bodies);
                w.get_mut(handle)
                    .set_position_wrt_parent(na::Isometry3::from_parts(
                        scaled_pos.to_na().into(),
                        rot.to_na(),
                    ));

                handle
            };
            (handle, world.clone())
        });

        // Set the parent-child link.
        self.children.push(child_collider_handle);
        child.parent = Some(NonNull::new(self as *mut _).unwrap());
    }

    /// Removes a shape from this compound shape, and changes it back to a singular shape if
    /// no children are left.
    ///
    /// This function assumes that `self` is not already a child.
    pub fn detach(&mut self, child: &mut RigidBody) {
        if child.parent.is_none() || child.collider.is_removed() {
            panic!("Child is not attached to parent.");
        }

        if child.parent.as_ref().unwrap().as_ptr() != (self as *mut RigidBody) {
            panic!("Child is not attached to this rigid body.");
        }

        // Convert current transform to world coordinates.
        let parent_transform = self.rigid_body.as_ref().position().clone();

        // TODO: Store child transform states so we can reconstruct when re-adding to the world.

        // Detach from parent by removing from the collider set.
        self.collider.set_removed(|handle, world| {
            // Compute the combined transform, then update the rigid body
            // transform of the child so it's in the right place once it's
            // re-added to the world.
            //
            // We need to do this before removing the collider from the world,
            // because position_wrt_parent() gets cleared.
            let combined_transform =
                parent_transform * world.as_ref().get(handle).position_wrt_parent().unwrap();
            child
                .rigid_body
                .as_mut()
                .set_position(combined_transform, true);

            // Remove from the world.
            let collider = {
                let w = &mut *world.as_mut();
                w.colliders
                    .remove(handle, &mut w.island_manager, &mut w.rigid_bodies, true)
                    .unwrap()
            };

            // Break parent-child link.
            self.children.swap_remove(
                self.children
                    .iter()
                    .position(|h| *h == handle)
                    .expect("child collider missing from children list"),
            );
            child.parent = None;

            collider
        });

        // Now we're in the 'removed' state.
    }

    /// Calculates the bounding box.
    #[bind(out_param = true)]
    pub fn get_bounding_box(&self) -> Box3 {
        let aabb = self.collider.as_ref().compute_aabb();
        Box3::new(
            Vec3::from_na_point(&aabb.mins),
            Vec3::from_na_point(&aabb.maxs),
        )
    }

    /// Calculates the compound bounding box.
    #[bind(out_param = true)]
    pub fn get_bounding_box_compound(&self) -> Box3 {
        if !self.is_parent() {
            panic!("Only enabled for parents");
        }

        if !self.collider.is_added() {
            panic!("Only enabled when added to the world");
        }

        let (collider_handle, world) = self.collider.added_as_ref().unwrap();
        let w = &mut *world.as_mut();

        // Get AABB of the main collider.
        let mut aabb = w.get(*collider_handle).compute_aabb();
        let parent_transform = w.get(*collider_handle).position().clone();

        // Incorporate the AABBs of the compound shapes.
        for child_collider_handle in self.children.iter() {
            let collider = w.get_mut(*child_collider_handle);
            let child_global_transform = parent_transform * collider.position_wrt_parent().unwrap();
            let child_aabb = collider.shape().compute_aabb(&child_global_transform);
            aabb.mins = aabb.mins.inf(&child_aabb.mins);
            aabb.maxs = aabb.maxs.sup(&child_aabb.maxs);
        }

        Box3::new(
            Vec3::from_na_point(&aabb.mins),
            Vec3::from_na_point(&aabb.maxs),
        )
    }

    /// Calculates the local bounding box.
    #[bind(out_param = true)]
    pub fn get_bounding_box_local(&self) -> Box3 {
        let aabb = self.collider.as_ref().shape().compute_local_aabb();
        Box3::new(
            Vec3::from_na_point(&aabb.mins),
            Vec3::from_na_point(&aabb.maxs),
        )
    }

    /// Calculates the local compound bounding box.
    #[bind(out_param = true)]
    pub fn get_bounding_box_local_compound(&self) -> Box3 {
        if !self.is_parent() {
            panic!("Only enabled for parents");
        }

        if !self.collider.is_added() {
            panic!("Only enabled when added to the world");
        }

        let (collider_handle, world) = self.collider.added_as_ref().unwrap();
        let w = &mut *world.as_mut();

        // Get AABB of the main collider.
        let mut aabb = w.get(*collider_handle).shape().compute_local_aabb();

        // Incorporate the AABBs of the compound shapes.
        for child_collider_handle in self.children.iter() {
            let collider = w.get_mut(*child_collider_handle);
            let child_aabb = collider
                .shape()
                .compute_aabb(collider.position_wrt_parent().unwrap());
            aabb.mins = aabb.mins.inf(&child_aabb.mins);
            aabb.maxs = aabb.maxs.sup(&child_aabb.maxs);
        }

        Box3::new(
            Vec3::from_na_point(&aabb.mins),
            Vec3::from_na_point(&aabb.maxs),
        )
    }

    pub fn get_bounding_radius(&self) -> f32 {
        self.get_bounding_box_local().half_extents().length()
    }

    pub fn get_bounding_radius_compound(&self) -> f32 {
        self.get_bounding_box_local_compound()
            .half_extents()
            .length()
    }

    pub fn get_speed(&self) -> f32 {
        self.rigid_body.as_ref().linvel().norm()
    }

    /// Returns the local -> world matrix for this rigid body.
    pub fn get_to_world_matrix(&self) -> Matrix {
        self.get_world_matrix_unscaled() * Matrix::from_scale(Vec3::splat(self.get_scale()))
    }

    /// Returns the world -> local matrix for this rigid body.
    pub fn get_to_local_matrix(&self) -> Matrix {
        self.get_to_world_matrix().inverse()
    }

    #[bind(out_param = true)]
    pub fn get_velocity(&self) -> Vec3 {
        Vec3::from_na(self.rigid_body.as_ref().linvel())
    }

    #[bind(name = "GetVelocityA", out_param = true)]
    pub fn get_angular_velocity(&self) -> Vec3 {
        Vec3::from_na(self.rigid_body.as_ref().angvel())
    }

    /// When disabled, the object will pass through others without colliding
    /// and will not be returned from ray or shape casts.
    pub fn set_collidable(&mut self, collidable: bool) {
        self.collidable = collidable;
        let collision_group = if self.collidable {
            self.collision_group
        } else {
            rp::InteractionGroups::none()
        };
        self.collider.as_mut().set_collision_groups(collision_group);
    }

    pub fn set_collision_group(&mut self, group: u32) {
        self.collision_group.memberships = group.into();
        let collision_group = if self.collidable {
            self.collision_group
        } else {
            rp::InteractionGroups::none()
        };
        self.collider.as_mut().set_collision_groups(collision_group);
    }

    pub fn set_collision_mask(&mut self, mask: u32) {
        self.collision_group.filter = mask.into();
        let collision_group = if self.collidable {
            self.collision_group
        } else {
            rp::InteractionGroups::none()
        };
        self.collider.as_mut().set_collision_groups(collision_group);
    }

    pub fn set_drag(&mut self, linear: f32, angular: f32) {
        let mut rb = self.rigid_body.as_mut();
        rb.set_linear_damping(linear);
        rb.set_angular_damping(angular);
    }

    pub fn set_friction(&mut self, friction: f32) {
        self.collider.as_mut().set_friction(friction);
    }

    pub fn set_kinematic(&mut self, kinematic: bool) {
        let body_type = if kinematic {
            rp::RigidBodyType::KinematicPositionBased
        } else {
            rp::RigidBodyType::Dynamic
        };
        self.rigid_body.as_mut().set_body_type(body_type, true);
    }

    pub fn set_restitution(&mut self, restitution: f32) {
        self.collider.as_mut().set_restitution(restitution);
    }

    pub fn set_sleep_threshold(&mut self, linear: f32, angular: f32) {
        let mut rb = self.rigid_body.as_mut();
        rb.activation_mut().linear_threshold = linear;
        rb.activation_mut().angular_threshold = angular;
    }

    pub fn get_mass(&self) -> f32 {
        self.rigid_body.as_ref().mass()
    }

    /// The mass of child objects does not affect the mass or inertia of the parent
    pub fn set_mass(&mut self, mass: f32) {
        self.rigid_body.as_mut().set_additional_mass(mass, true);
    }

    /// Children return the parent position.
    #[bind(name = "GetPos", out_param = true)]
    pub fn get_position(&self) -> Vec3 {
        self.get_world_matrix_unscaled().get_translation()
    }

    /// Local coordinates are relative to the parent *before* scaling.
    #[bind(name = "GetPosLocal", out_param = true)]
    pub fn get_position_local(&self) -> Vec3 {
        if self.parent.is_some() {
            Vec3::from_na(
                &self
                    .collider
                    .as_ref()
                    .position_wrt_parent()
                    .unwrap()
                    .translation
                    .vector,
            )
        } else {
            Vec3::ZERO
        }
    }

    #[bind(name = "SetPos")]
    pub fn set_position(&mut self, pos: &Vec3) {
        self.rigid_body.as_mut().set_translation(pos.to_na(), true);
    }

    /// Local coordinates are relative to the parent *before* scaling. The
    /// given position will be multiplied by the parent's scale.
    #[bind(name = "SetPosLocal")]
    pub fn set_position_local(&mut self, pos: &Vec3) {
        if self.parent.is_some() {
            let mut isometry = self
                .collider
                .as_ref()
                .position_wrt_parent()
                .unwrap()
                .clone();
            isometry.translation.vector = Vec3::to_na(pos);
            self.collider.as_mut().set_position_wrt_parent(isometry);
        }
    }

    #[bind(name = "GetRot", out_param = true)]
    pub fn get_rotation(&self) -> Quat {
        Quat::from_mat4(&self.get_world_matrix_unscaled())
    }

    #[bind(name = "GetRotLocal", out_param = true)]
    pub fn get_rotation_local(&mut self) -> Quat {
        if self.parent.is_some() {
            Quat::from_na(
                &self
                    .collider
                    .as_ref()
                    .position_wrt_parent()
                    .unwrap()
                    .rotation,
            )
        } else {
            Quat::IDENTITY
        }
    }

    #[bind(name = "SetRot")]
    pub fn set_rotation(&mut self, rot: &mut Quat) {
        self.rigid_body.as_mut().set_rotation(rot.to_na(), true);
    }

    #[bind(name = "SetRotLocal")]
    pub fn set_rotation_local(&mut self, rot: &Quat) {
        if self.parent.is_some() {
            let mut isometry = self
                .collider
                .as_ref()
                .position_wrt_parent()
                .unwrap()
                .clone();
            isometry.rotation = Quat::to_na(rot);
            self.collider.as_mut().set_position_wrt_parent(isometry);
        }
    }

    pub fn get_scale(&self) -> f32 {
        self.shape_scale
    }

    /// When called on a parent object the positions of all children will be
    /// multiplied such that they retain the same relative position. Child
    /// scale is not affected by parent scale (i.e. it is not inherited).
    pub fn set_scale(&mut self, scale: f32) {
        let scaled_shape = CollisionShape::new(scale, self.shape_type.clone());

        // Replace the shape of the current collider by cloning the reference counted Shape object.
        self.collider.as_mut().set_shape(rp::SharedShape(
            scaled_shape.collider.shared_shape().0.clone(),
        ));

        self.shape_type = scaled_shape.shape;
        self.shape_scale = scale;
    }
}
