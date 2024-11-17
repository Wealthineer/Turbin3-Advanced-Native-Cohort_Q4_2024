use pinocchio::{account_info::AccountInfo, pubkey::Pubkey};

// #[repr(C)]
// #[derive(Clone, Copy, Pod, Zeroable)]
// pub struct Escrow {
//     pub maker: Pubkey,
//     pub mint_a: Pubkey,
//     pub mint_b: Pubkey,
//     pub receive: u64,
//     pub bump: u64,
// }

pub struct Escrow(*const u8);

impl Escrow {
    pub const LEN: usize = 32 + 32 + 32 + 8 + 8;

    pub fn from_account_info_unchecked(account_info: &AccountInfo) -> Self {
        unsafe { Self(account_info.borrow_data_unchecked().as_ptr()) }
    }

    pub fn from_account_info(account_info: &AccountInfo) -> Self {
        assert_eq!(account_info.data_len(), Self::LEN);
        assert_eq!(account_info.owner(), &crate::ID);
        Self::from_account_info_unchecked(account_info)
    }

    pub fn maker(&self) -> Pubkey {
        unsafe  { *(self.0 as *const Pubkey) }
    }

    pub fn mint_a(&self) -> Pubkey {
        unsafe { *(self.0.add(32) as *const Pubkey) }
    }

    pub fn mint_b(&self) -> Pubkey {
        unsafe { *(self.0.add(64) as *const Pubkey) }
    }

    pub fn receive(&self) -> u64 {
        unsafe { *(self.0.add(96) as *const u64) }
    }

    pub fn bump(&self) ->  u8 {
        unsafe { *(self.0.add(104) as *const u8) }
    }
}
