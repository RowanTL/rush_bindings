use pyo3::prelude::*;

const CODE_BLOCK_STR: &str = "<class 'pyshgp.push.atoms.CodeBlock'>";

#[pymodule]
mod rush_bindings {
    use pyo3::{prelude::*, types::PyDict};
    use rush::push::state::{Gene, PushState};

    /// Later, this function will need to accept inputs in its own
    /// parameter.
    /// Can kinda just take the module as input. Cool.
    #[pyfunction]
    fn pront(x: Bound<'_, PyDict>, py: Python<'_>) -> PyResult<()> {
        // println!("{:?}", x.get_type());
        // println!("{:?}", x.get_item("exec")?.unwrap().get_type().to_string());
        let rust_state: PushState = x.extract()?;
        let genome = x.get_item("exec")?.unwrap();
        rec_block(&genome)?;
        //     for gene in genome_iter {
        //         let temp_gene = &gene?;
        //         println!("{:?}", temp_gene);
        //         if let Ok(block) = temp_gene.try_iter() {
        //             println!("{:?}", );
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

    fn rec_block(block: &Bound<'_, PyAny>) -> PyResult<Vec<Gene>> {
        let block_iter = block.try_iter()?;
        for gene in block_iter {
            let temp_gene = gene?;
            if temp_gene.to_string().starts_with("InstructionMeta") {
                println!("{:?}", temp_gene.getattr("name")?.to_string());
            }
        }
        Ok(Vec::new())
    }
}
