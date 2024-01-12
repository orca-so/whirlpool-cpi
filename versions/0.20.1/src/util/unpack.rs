use anchor_lang::prelude::*;

use crate::state::{TickArray, Tick, TICK_ARRAY_SIZE_USIZE, NUM_REWARDS};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct UnpackedTickArray {
  pub start_tick_index: i32,
  pub ticks: [UnpackedTick; TICK_ARRAY_SIZE_USIZE],
  pub whirlpool: Pubkey,
}

#[derive(borsh::BorshDeserialize, Debug, PartialEq, Default, Clone, Copy)]
pub struct UnpackedTick {
    pub initialized: bool,
    pub liquidity_net: i128,
    pub liquidity_gross: u128,
    pub fee_growth_outside_a: u128,
    pub fee_growth_outside_b: u128,
    pub reward_growths_outside: [u128; NUM_REWARDS],
}

pub fn unpack_tick_array(loader: &AccountLoader<TickArray>) -> std::result::Result<Box<UnpackedTickArray>, ProgramError> {
  let loaded = loader.load()?;

  let boxed = Box::new(UnpackedTickArray {
    start_tick_index: loaded.start_tick_index,
    ticks: loaded.ticks.iter().map(|t| UnpackedTick {
      initialized: t.initialized,
      liquidity_net: t.liquidity_net,
      liquidity_gross: t.liquidity_gross,
      fee_growth_outside_a: t.fee_growth_outside_a,
      fee_growth_outside_b: t.fee_growth_outside_b,
      reward_growths_outside: t.reward_growths_outside,
    }).collect::<Vec<_>>().try_into().unwrap(),
    whirlpool: loaded.whirlpool,
  });

  Ok(boxed)
}
