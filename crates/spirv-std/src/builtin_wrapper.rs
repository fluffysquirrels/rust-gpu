use glam::UVec3;

/// TODO
#[spirv(builtin_wrapper("local_invocation_index"))]
#[repr(transparent)]
pub struct LocalInvocationIndex(pub u32);

/// TODO
#[spirv(builtin_wrapper("local_invocation_id"))]
#[repr(transparent)]
pub struct LocalInvocationId(pub UVec3);
