use crate::tools::blocker::ipv4_blocker::ipv4_blocker::Ipv4Blocker;
use crate::SYS;
use crate::tools::file::get_path::get_current_path;

pub mod ipv4_blocker;


/// ipv4黑白名单拦截器
pub struct BlackWhiteListV4 {
    black_list:Ipv4Blocker,
    white_list:Ipv4Blocker
}


impl Clone for BlackWhiteListV4 {
    fn clone(&self) -> Self {
        Self {
            black_list: self.black_list.clone(),
            white_list: self.white_list.clone(),
        }
    }

}


impl BlackWhiteListV4 {

    /// 创建 ipv4黑白名单拦截器
    pub fn new(black_list_file_arg:&Option<String>,
                white_list_file_arg:&Option<String>, is_source:bool) -> Self {

        let (black_list_file, white_list_file) = if is_source {
            get_default_source_black_white_list(black_list_file_arg, white_list_file_arg)
        } else {
            get_default_destination_black_white_list(black_list_file_arg, white_list_file_arg)
        };

        Self {
            black_list: Ipv4Blocker::new(black_list_file),
            white_list: Ipv4Blocker::new(white_list_file),
        }

    }


    /// 判断 ip地址 是否因为 黑白名单 被限制
    /// 如果 ip地址 可用, 返回true; 如果 ip地址 不可用, 返回 false;
    pub fn ip_is_avail(&self, addr:u32) -> bool {

        if self.black_list.ip_is_marked(addr) {

            // 如果被黑名单标记, 表示被黑名单禁止
            return if self.white_list.ip_is_marked(addr) {

                // 虽然被黑名单禁止, 但是被 白名单 标记, 表示其无论如何也被允许
                true
            } else {

                // 被黑名单禁止, 也不在白名单的保护范围, 就说明 地址不可用
                false
            }

        }

        // 没被禁止, 就是允许
        true
    }

    /// 判断 ip地址 是否因为 黑白名单 被限制
    /// 如果 ip地址 被阻止, 返回true; 如果 ip地址 没被阻止, 返回 false;
    #[allow(dead_code)]
    pub fn ip_is_blocked(&self, addr:u32) -> bool {

        if self.black_list.ip_is_marked(addr) {

            // 如果被黑名单标记, 表示被黑名单禁止
            return if self.white_list.ip_is_marked(addr) {

                // 虽然被黑名单禁止, 但是被 白名单 标记, 表示其无论如何也 不被阻止
                false
            } else {

                // 被黑名单禁止, 也不在白名单的保护范围, 就说明 地址被阻止
                true
            }

        }

        // 没被黑名单阻止
        false
    }

    /// 使用当前局部探测范围, 生成局部约束(黑白名单)
    pub fn gen_local_constraints(&self, start:u32, end:u32) -> Self {

        Self {
            black_list: self.black_list.set_tar_range(start, end),
            white_list: self.white_list.set_tar_range(start, end),
        }
    }

    /// 手动对黑白名单进行清空
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.black_list.clear(false);
        self.white_list.clear(false);
    }



}


fn get_default_destination_black_white_list(black_list_file_arg:&Option<String>,
                                            white_list_file_arg:&Option<String>) -> (String, String) {

    let black_list_file;
    let white_list_file;

    if let Some(black_path) = black_list_file_arg {
        black_list_file = black_path.to_string();
    } else {
        black_list_file = get_current_path(&SYS.get_info("conf", "destination_black_list_v4"));
    }

    if let Some(white_path) = white_list_file_arg {
        white_list_file = white_path.to_string();
    } else {
        white_list_file = get_current_path(&SYS.get_info("conf", "destination_white_list_v4"));
    }

    (black_list_file, white_list_file)
}


fn get_default_source_black_white_list(black_list_file_arg:&Option<String>,
                                            white_list_file_arg:&Option<String>) -> (String, String) {

    let black_list_file;
    let white_list_file;

    if let Some(black_path) = black_list_file_arg {
        black_list_file = black_path.to_string();
    } else {
        black_list_file = get_current_path(&SYS.get_info("conf", "source_black_list_v4"));
    }

    if let Some(white_path) = white_list_file_arg {
        white_list_file = white_path.to_string();
    } else {
        white_list_file = get_current_path(&SYS.get_info("conf", "source_white_list_v4"));
    }

    (black_list_file, white_list_file)
}

