use std::any::Any;

pub trait Item:
    std::fmt::Debug + serde_traitobject::Serialize + serde_traitobject::Deserialize
{
    fn is_task(&self) -> bool;
    fn as_any(&self) -> &dyn Any;
}
