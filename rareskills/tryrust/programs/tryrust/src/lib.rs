use anchor_lang::prelude::*;

declare_id!("2SxGRHJpohacbkgb8DQJYcGi7dAtDgQ8R3n2C8au9VDW");

#[program]
pub mod tryrust {
    use std::collections::HashMap;

    use super::*;

    pub fn initialize(ctx: Context<Initialize>, key: String, value: String) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);

        for i in 0..10 {
            msg!("{:?}", i);
        }

        for i in (0..10).step_by(2) {
            msg!("{:?}", i);
        }

        let my_array: [u32; 5] = [1, 2, 3, 4, 5];
        let first_element = my_array[0];
        let third_element = my_array[2];
        let mut mutable_array: [u32; 3] = [10, 20, 30];
        mutable_array[1] = 25;

        let mut dynamic_array: Vec<u32> = Vec::new();
        dynamic_array.push(100);
        dynamic_array.push(200);
        dynamic_array.push(300);
        let first_element = dynamic_array[0];
        let third_element = dynamic_array[2];

        let mut my_map = HashMap::new();
        my_map.insert(key.to_string(), value.to_string());
        msg!("My name is {}", my_map[&key]);


        Ok(())
    }

    pub fn age_checker(ctx: Context<Initialize>, age: u64) -> Result<()> {
        // if age < 18 {
        //     msg!("You are a minor.");
        // } else {
        //     msg!("You are an adult.");
        // }

        // let result = if age >= 18 {"You are 18 years old or older"} else {"You are below 18 years old"};
        // msg!("{:?}", result);

        match age {
            1 => {
                msg!("The age is 1");
            },
            2 | 3 => {
                msg!("The age is either 2 or 3");
            },
            4..=6 => {
                msg!("The age is between 4 and 6");
            },
            _ => {
                msg!("The age is greater than 6");
            }
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
