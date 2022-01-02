use k_clustering::*;

fn main() {
    // let img = img.grayscale();
    let name = "in.jpg";
    for i in 0..50 {
        let img = image::open(name).unwrap();
        // let img = img.blur(2.);
        let mut kms = KMeansSegmentation::new(img, i);
        while (kms.iteration()) > 0 {}
        kms.overlay_color().save(format!("out{}.png", i)).unwrap();
    }
}
