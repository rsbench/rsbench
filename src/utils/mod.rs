pub mod color;
pub mod report;
pub mod term;

use crate::unlock_test::Service;

impl PartialEq for Box<dyn Service + Send + Sync> {
    // 定义 eq 方法，比较两个动态盒子的名称是否相等
    fn eq(&self, other: &Self) -> bool {
        // 调用 name 方法获取服务名称，并比较是否相等
        self.name() == other.name()
    }
}
