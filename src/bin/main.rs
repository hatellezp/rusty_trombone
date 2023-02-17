use trombone::io_utils;
use trombone::sample_generator;

fn main() {
    let filename_in = "sample2.wav";
    let filename_out = "sample2_changed.wav";

    let reader = io_utils::read(filename_in);
    let spec = reader.spec();

    // convert to a vec and modify
    let mut data = io_utils::to_vec::<i16>(reader, None);
    for i in 0..data.len() {
        data[i] += (3000 / (i + 33)) as i16;

        if i % 333 == 21 {
            data[i] = 21;
        }
    }

    // write to file
    let mut sg = sample_generator::SGfromArray::new(&data);
    let res = io_utils::write(filename_out, spec, &mut sg, data.len());

    println!("writing result: {:?}", res);
}
