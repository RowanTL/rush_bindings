use pyo3::prelude::*;

#[pymodule]
mod rush_bindings {
    use pyo3::{prelude::*, types::PyDict};
    use rush::push::state::PushState;

    /// Later, this function will need to accept inputs in its own
    /// parameter.
    #[pyfunction]
    fn pront(x: Bound<'_, PyDict>) -> PyResult<()> {
        let rust_state: PushState = x.extract()?;
        // let genome = x.get_item("exec")?.unwrap();
        // for gene in &(genome.try_iter()) {}
        // if let Ok(genome_iter) = &(genome.try_iter()) {
        //     for gene in genome_iter {
        //         let temp_gene = &gene?;
        //         println!("{:?}", temp_gene);
        //         if let Ok(block) = temp_gene.try_iter() {
        //             println!("block found");
        //         }
        //         let temp_str = temp_gene.to_string();
        //         if temp_str.starts_with("InstructionMeta") {
        //             println!("InstructionMeta found")
        //         } else if temp_str.starts_with("Literal") {
        //             println!("Literal found")
        //         } else if temp_str.starts_with("Input") {
        //             println!("Input found")
        //         }
        //     }
        // }
        Ok(())
    }
}
