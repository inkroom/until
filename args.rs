

#[derive(Debug)]
pub(crate) struct Args {
    /**
     * 重试次数，不包括第一次，默认-1，一直重试
     */
    pub(crate) max: i32,
    /**
     * 期望的退出码，默认为0
     */
    pub(crate) code: i32,
}

impl Default for Args {
    fn default() -> Self {
        Args { max: -1, code: 0 }
    }
}

