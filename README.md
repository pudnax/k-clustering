# Value Sketches Using k-Means Clustering

## Usage

```Rust
use k_clustering::*;

fn main() {
    let img = image::open("p5.jpg").unwrap();

    // let img = img.grayscale();
    let img = img.blur(2.);

    let mut kms = KMeansSegmentation::new(img, 5);
    while (kms.iteration()) > 0 {}
    kms.overlay_color().save("out2.png").unwrap();
}
```

<p>
 <img src=./p5.jpg width="200" height="200">
 <img src=./out2.png width="200" height="200">
 <img src=./out3.png width="200" height="200">
</p>