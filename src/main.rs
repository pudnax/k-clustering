extern crate image;
use image::GenericImage;
use image::GenericImageView;

pub use cgmath::num_traits::NumCast;

fn main() {
    let img = image::open("p5.jpg").unwrap();

    // let img = img.grayscale();
    let img = img.blur(2.);

    let mut kms = KMeansSegmentation::new(img, 37);
    while (kms.iteration()) > 0 {}
    kms.overlay_color().save("out2.png").unwrap();
}

struct KMeansSegmentation {
    n_clusters: usize,
    image: image::DynamicImage,
    mean_colors: Vec<[u8; 3]>,
    cluster_assignment: Vec<Vec<Option<usize>>>,
}

impl KMeansSegmentation {
    fn new(image: image::DynamicImage, n_clusters: usize) -> KMeansSegmentation {
        let mean_colors = vec![[255, 255, 255]; n_clusters];
        let (w, h) = image.dimensions();
        let cluster_assignment = vec![vec![None; h as usize]; w as usize];
        KMeansSegmentation {
            n_clusters,
            image,
            mean_colors,
            cluster_assignment,
        }
    }

    fn cdist(a: [u8; 3], b: [u8; 3]) -> f64 {
        let r1 = a[0] as f64;
        let g1 = a[1] as f64;
        let b1 = a[2] as f64;
        let r2 = b[0] as f64;
        let g2 = b[1] as f64;
        let b2 = b[2] as f64;
        ((r1 - r2).powi(2) + (g1 - g2).powi(2) + (b1 - b2).powi(2)).sqrt()
    }

    fn cdist2(a: [u8; 3], b: [u8; 3]) -> f64 {
        let r1 = a[0] as f64;
        let g1 = a[1] as f64;
        let b1 = a[2] as f64;
        let r2 = b[0] as f64;
        let g2 = b[1] as f64;
        let b2 = b[2] as f64;
        let rbar = (r1 + r2) / 2.;
        let deltar = r1 - r2;
        let deltag = g1 - g2;
        let deltab = b1 - b2;
        (2. * deltar * deltar
            + 4. * deltag * deltag
            + 3. * deltab * deltab
            + (rbar + (deltar * deltar - deltab * deltab)) / 256.)
            .sqrt()
    }

    fn iteration(&mut self) -> usize {
        let mut changed = 0;
        let mut totals = vec![1.; self.n_clusters];
        let mut ctotals = vec![[0., 0., 0.]; self.n_clusters];

        let (w, h) = self.image.dimensions();

        for x in 0..w as usize {
            for y in 0..h as usize {
                let curr_cluster = self.cluster_assignment[x][y];

                let pixel = self.image.get_pixel(x as u32, y as u32);
                let pixel_color = [pixel[0], pixel[1], pixel[2]];

                let mut closest_dist = std::f64::MAX;
                let mut closest_cluster: Option<usize> = None;

                for i in 0..self.n_clusters {
                    let cluster_color = self.mean_colors[i];

                    let dist = KMeansSegmentation::cdist(pixel_color, cluster_color);
                    if dist < closest_dist {
                        closest_dist = dist;
                        closest_cluster = Some(i);
                    }
                }

                self.cluster_assignment[x][y] = closest_cluster;

                if let Some(closest_cluster) = closest_cluster {
                    ctotals[closest_cluster][0] += pixel_color[0] as f64;
                    ctotals[closest_cluster][1] += pixel_color[1] as f64;
                    ctotals[closest_cluster][2] += pixel_color[2] as f64;
                    totals[closest_cluster] += 1.;
                }

                if closest_cluster != curr_cluster {
                    changed += 1;
                }
            }
        }
        for i in 0..self.n_clusters {
            self.mean_colors[i] = [
                (ctotals[i][0] / totals[i]) as u8,
                (ctotals[i][1] / totals[i]) as u8,
                (ctotals[i][2] / totals[i]) as u8,
            ];
        }

        changed
    }

    fn overlay(&self) -> image::DynamicImage {
        let (w, h) = self.image.dimensions();
        // let o = image::ImageBuffer::new(w, h);
        let mut o = image::DynamicImage::new_rgb8(w, h);

        let (mut min, mut max) = (255, 0);
        for k in 0..self.n_clusters {
            let c = self.mean_colors[k];
            if c[0] < min {
                min = c[0];
            }
            if c[0] > max {
                max = c[0];
            }
        }

        for x in 0..w as usize {
            for y in 0..h as usize {
                let mut grayvalue = 127;
                if let Some(cluster) = self.cluster_assignment[x][y] {
                    grayvalue = self.mean_colors[cluster][0];
                }

                let mut c = [
                    map_range(grayvalue, min, max, 0, 255),
                    map_range(grayvalue, min, max, 0, 255),
                    map_range(grayvalue, min, max, 0, 255),
                    255,
                ];

                if grayvalue != min && grayvalue != max {
                    c = [127, 127, 127, 255];
                }

                o.put_pixel(x as u32, y as u32, image::Rgba(c));
            }
        }

        o
    }

    fn overlay_color(&self) -> image::DynamicImage {
        let (w, h) = self.image.dimensions();
        let mut o = image::DynamicImage::new_rgb8(w, h);

        for x in 0..w as usize {
            for y in 0..h as usize {
                let mut color = [0, 0, 0];
                if let Some(cluster) = self.cluster_assignment[x][y] {
                    color = self.mean_colors[cluster];
                }
                o.put_pixel(
                    x as u32,
                    y as u32,
                    image::Rgba([color[0], color[1], color[2], 255]),
                );
            }
        }

        o
    }
}

pub fn map_range<X, Y>(val: X, in_min: X, in_max: X, out_min: Y, out_max: Y) -> Y
where
    X: NumCast,
    Y: NumCast,
{
    macro_rules! unwrap_or_panic {
        ($result:expr, $arg:expr) => {
            $result.unwrap_or_else(|| panic!("[map_range] failed to cast {} arg to `f64`"))
        };
    }

    let val_f: f64 = unwrap_or_panic!(NumCast::from(val), "first");
    let in_min_f: f64 = unwrap_or_panic!(NumCast::from(in_min), "second");
    let in_max_f: f64 = unwrap_or_panic!(NumCast::from(in_max), "third");
    let out_min_f: f64 = unwrap_or_panic!(NumCast::from(out_min), "fourth");
    let out_max_f: f64 = unwrap_or_panic!(NumCast::from(out_max), "fifth");

    NumCast::from((val_f - in_min_f) / (in_max_f - in_min_f) * (out_max_f - out_min_f) + out_min_f)
        .unwrap_or_else(|| panic!("[map_range] failed to cast result to target type"))
}
