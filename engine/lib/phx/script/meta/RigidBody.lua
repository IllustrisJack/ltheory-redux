---@meta

RigidBody = RigidBody

---@return RigidBody
function RigidBody:CreateBox() end

---@param mesh Mesh
---@return RigidBody
function RigidBody:CreateBoxFromMesh(mesh) end

---@return RigidBody
function RigidBody:CreateSphere() end

---@param mesh Mesh
---@return RigidBody
function RigidBody:CreateSphereFromMesh(mesh) end

---@param mesh Mesh
---@return RigidBody
function RigidBody:CreateConvexHullFromMesh(mesh) end

---@param mesh Mesh
---@return RigidBody
function RigidBody:CreateConvexDecompositionFromMesh(mesh) end

---@param mesh Mesh
---@return RigidBody
function RigidBody:CreateTrimeshFromMesh(mesh) end

---Return a reference to the parent rigid body, that we can guarantee
---has a lifetime as long as self.
---@return RigidBody
function RigidBody.getParentBody(self) end

---@param force Vec3
function RigidBody.applyForce(self, force) end

---@param torque Vec3
function RigidBody.applyTorque(self, torque) end

---Adds another rigid body as a child of this rigid body. This means that
---the child's position will be controlled by `self`.
---
---Only a single level of attachment is supported. Child objects do not
---affect the mass or inertia of the parent. Position is relative to the
---unscaled parent. i.e. it will be multiplied by the current scale. This
---function is O(1). Warning: if one object is attached to another and a
---third object happens to be between them this may trap the third object.
---The same issue may occur when spawning one compound inside another.
---
---This function expects that the child is not already in the physics
---world, as it will add it if the parent is already in the world.
---
---This function assumes that `self` is not already a child.
---@param child RigidBody
---@param pos Vec3
---@param rot Quat
function RigidBody.attach(self, child, pos, rot) end

---Removes a rigid body as a child of this rigid body. This means that
---the child's will be under control of it's own position.
---
---This function will result in a child that is not in the world anymore,
---so it will need to be re-added with physics.add_rigid_body(...).
---
---This function assumes that `self` is not already a child.
---@param child RigidBody
function RigidBody.detach(self, child) end

---Calculates the bounding box.
---@param result Box3 [out]
function RigidBody.getBoundingBox(self, result) end

---Calculates the compound bounding box.
---@param result Box3 [out]
function RigidBody.getBoundingBoxCompound(self, result) end

---Calculates the local bounding box.
---@param result Box3 [out]
function RigidBody.getBoundingBoxLocal(self, result) end

---Calculates the local compound bounding box.
---@param result Box3 [out]
function RigidBody.getBoundingBoxLocalCompound(self, result) end

---@return number
function RigidBody.getBoundingRadius(self) end

---@return number
function RigidBody.getBoundingRadiusCompound(self) end

---@return number
function RigidBody.getSpeed(self) end

---Returns the local -> world matrix for this rigid body.
---@return Matrix
function RigidBody.getToWorldMatrix(self) end

---Returns the world -> local matrix for this rigid body.
---@return Matrix
function RigidBody.getToLocalMatrix(self) end

---@param result Vec3 [out]
function RigidBody.getVelocity(self, result) end

---@param result Vec3 [out]
function RigidBody.getVelocityA(self, result) end

---When disabled, the object will pass through others without colliding
---and will not be returned from ray or shape casts.
---@param collidable boolean
function RigidBody.setCollidable(self, collidable) end

---@param group integer
function RigidBody.setCollisionGroup(self, group) end

---@param mask integer
function RigidBody.setCollisionMask(self, mask) end

---@param linear number
---@param angular number
function RigidBody.setDrag(self, linear, angular) end

---@param friction number
function RigidBody.setFriction(self, friction) end

---@param kinematic boolean
function RigidBody.setKinematic(self, kinematic) end

---@param restitution number
function RigidBody.setRestitution(self, restitution) end

---@param linear number
---@param angular number
function RigidBody.setSleepThreshold(self, linear, angular) end

---@return number
function RigidBody.getMass(self) end

---The mass of child objects does not affect the mass or inertia of the parent
---@param mass number
function RigidBody.setMass(self, mass) end

---Children return the parent position.
---@param result Vec3 [out]
function RigidBody.getPos(self, result) end

---Local coordinates are relative to the parent *before* scaling.
---@param result Vec3 [out]
function RigidBody.getPosLocal(self, result) end

---@param pos Vec3
function RigidBody.setPos(self, pos) end

---Local coordinates are relative to the parent *before* scaling. The
---given position will be multiplied by the parent's scale.
---@param pos Vec3
function RigidBody.setPosLocal(self, pos) end

---@param result Quat [out]
function RigidBody.getRot(self, result) end

---@param result Quat [out]
function RigidBody.getRotLocal(self, result) end

---@param rot Quat
function RigidBody.setRot(self, rot) end

---@param rot Quat
function RigidBody.setRotLocal(self, rot) end

---@return number
function RigidBody.getScale(self) end

---When called on a parent object the positions of all children will be
---multiplied such that they retain the same relative position. Child
---scale is not affected by parent scale (i.e. it is not inherited).
---@param scale number
function RigidBody.setScale(self, scale) end

---@param target RigidBody
---@return number
function RigidBody.distanceTo(self, target) end

---@return boolean
function RigidBody.isSleeping(self) end

