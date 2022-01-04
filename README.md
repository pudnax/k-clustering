# Value Sketches Using k-Means Clustering
https://sighack.com/post/generative-value-sketches-using-k-means-clustering

## Usage

```Rust
use k_clustering::*;

fn main() {
    let img = image::open("in.jpg").unwrap();

    // let img = img.grayscale();
    let img = img.blur(2.);

    let mut kms = KMeansSegmentation::new(img, 5);
    while (kms.iteration()) > 0 {}
    kms.overlay_color().save("out2.png").unwrap();
}
```

<p>
 <img src=./in.jpg width="200" height="200">
 <img src=./out2.png width="200" height="200">
 <img src=./out3.png width="200" height="200">
</p>
