extern crate structopt;
#[macro_use]
extern crate trackable;

use std::mem;
use structopt::StructOpt;

use cannyls::block::BlockSize;
use cannyls::lump::*;
use cannyls::nvm::FileNvm;
use cannyls::storage::StorageBuilder;
use cannyls::Error;

#[derive(StructOpt, Debug)]
#[structopt(name = "get_unexpected_data")]
struct Opt {
    #[structopt(long = "phase")]
    phase: usize,
}

fn main() -> Result<(), Error> {
    let path = "test.lusf";
    let opt = Opt::from_args();

    let lump_id1 = LumpId::new(1_111);
    let lump_id2 = LumpId::new(22_222_222);

    if opt.phase == 1 {
        let nvm = track!(FileNvm::create(
            path,
            BlockSize::min().ceil_align(1024 * 1024)
        ))?;
        let mut builder = StorageBuilder::new();
        builder.enable_safe_release_mode();
        let mut storage = track!(builder.create(nvm))?;

        let lump_data1 = track!(storage.allocate_lump_data_with_bytes(b"hoge"))?;

        println!("put `hoge` to lump_id1(= {:?}).", lump_id1);
        track!(storage.put(&lump_id1, &lump_data1))?;
        track!(storage.journal_sync())?;

        println!("delete lump_id1(= {:?}).", lump_id1);
        track!(storage.delete(&lump_id1))?;

        println!("put `foo` to lump_id2(= {:?}).", lump_id2);
        let lump_data2 = track!(storage.allocate_lump_data_with_bytes(b"foo"))?;
        track!(storage.put(&lump_id2, &lump_data2))?;

        mem::forget(storage);
        Ok(())
    } else if opt.phase == 2 {
        let nvm = track!(FileNvm::open(path))?;
        let mut builder = StorageBuilder::new();
        builder.enable_safe_release_mode();
        let mut storage = track!(builder.open(nvm))?;

        println!("try to read a datum from lump_id1(= {:?}).", lump_id1);
        let v = track!(storage.get(&lump_id1))?;
        if let Some(v) = v {
            println!("We deleted lump_id1(= {:?}); however, we can read a datum from lump_id1.", lump_id1);
            let datam = String::from_utf8_lossy(v.as_bytes());
            if datam != "hoge" {
                println!("Furthermore, the read data `{}` is not `hoge`.", datam);
            } else {
                println!("The read data is `hoge`");
            }
        } else {
            panic!("unexpected behaviour for v0.9.3");
        }
        Ok(())
    } else {
        panic!("--phase=1 or --phase=2 are only allowed");
    }
}
