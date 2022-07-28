mod test_util;

#[cfg(test)]
mod inventory {
    use super::test_util;
    use rscache::extensions::osrs::InventoryLoader;

    fn inventory_loader() -> InventoryLoader {
        InventoryLoader::new(&test_util::osrs_cache()).unwrap()
    }

    #[test]
    fn load_player_backpack() {
        let inventory_loader = inventory_loader();

        let inventory = inventory_loader.load(93).unwrap();

        assert_eq!(28, inventory.capacity.unwrap());
    }
}

#[cfg(test)]
mod varbits {
    use super::test_util;
    use rscache::extensions::osrs::VarbitLoader;

    fn varbit_loader() -> VarbitLoader {
        VarbitLoader::new(&test_util::osrs_cache()).unwrap()
    }

    #[test]
    fn load_sample_varbit() {
        let varbit_loader = varbit_loader();

        let chatbox_varbit = varbit_loader.load(8119).unwrap();

        assert_eq!(1737, chatbox_varbit.varp_id);
        assert_eq!(31, chatbox_varbit.least_significant_bit);
        assert_eq!(31, chatbox_varbit.most_significant_bit);
    }
}
