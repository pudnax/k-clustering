use k_clustering::*;

fn main() {
    let img = image::open("in.jpg").unwrap();

    // let img = img.grayscale();
    let img = img.blur(2.);

    let mut kms = KMeansSegmentation::new(img, 5);
    while (kms.iteration()) > 0 {}
    kms.overlay_color().save("out.png").unwrap();
}
