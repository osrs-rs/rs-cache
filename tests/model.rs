mod common;

mod osrs {
    use super::common;
    
    #[test]
    fn load_model_something() -> rscache::Result<()> {
        let _cache = common::osrs::setup()?;
        // let model_loader = common::osrs::load_models(&cache)?;
        
        // // let model = model_loader.load(1042).unwrap();
        // for (id, model) in &model_loader {
            
        // }

        //panic!();
        
        Ok(())
    }
}

mod rs3 {
}