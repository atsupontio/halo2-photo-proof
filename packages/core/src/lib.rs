use std::{marker::PhantomData};

use halo2_proofs::{
    arithmetic::FieldExt,
    circuit::SimpleFloorPlanner,
    circuit::{AssignedCell, Layouter, Value},
    pasta::{group::ff::PrimeField, EqAffine, Fp},
    plonk::*,
    poly::{commitment::Params, Rotation},
    transcript::{Blake2bRead, Blake2bWrite, Challenge255},
};
use image::{EncodableLayout, ImageBuffer, Rgba};
use rand_core::OsRng;
pub struct Parameter {
    parameter: Params<EqAffine>,
}
impl Parameter {
    fn set_parameter(param: Params<EqAffine>) -> Self {
        let k = *K.get().unwrap();
        let params: Params<EqAffine> = Params::new(k);
        Parameter { parameter: params }
    }

    fn get_parameter(&self) -> Params<EqAffine> {
        self.parameter.clone()
    }
}

use once_cell::sync::{Lazy, OnceCell};
pub static PARAMETER: Lazy<Parameter> = Lazy::new(|| setup());

pub static K: OnceCell<u32> = OnceCell::new();
pub static WIDTH: OnceCell<usize> = OnceCell::new();
pub static HEIGHT: OnceCell<usize> = OnceCell::new();
pub static S_WIDTH: OnceCell<usize> = OnceCell::new();
pub static S_HEIGHT: OnceCell<usize> = OnceCell::new();

pub fn setup() -> Parameter {
    let _ =  K.set(10);
    let k = *K.get().unwrap();
    let params: Params<EqAffine> = Params::new(k);
    Parameter { parameter: params }
}

pub fn exec_mosaic(
    buf: Vec<u8>,
    scale_factor: u32,
    width: u32,
    height: u32,
) -> Vec<u8>
{
    let mut img: ImageBuffer<Rgba<u8>, Vec<_>> = ImageBuffer::from_raw(width, height, buf).unwrap();

    let block_size = scale_factor;

    let new_width = width / block_size;
    let new_height = height / block_size;

    let mut new_img = ImageBuffer::new(new_width, new_height);

    let mut red_r = Vec::new();
    let mut green_r = Vec::new();
    let mut blue_r = Vec::new();
    let mut alpha_r = Vec::new();

    for y in 0..new_height {
        for x in 0..new_width {
            let mut r_sum: u32 = 0;
            let mut g_sum: u32 = 0;
            let mut b_sum: u32 = 0;
            let mut a_sum: u32 = 0;

            for dy in 0..block_size {
                for dx in 0..block_size {
                    // 元の画像の座標
                    // x, y は縮小後の画像(ブロック)の座標
                    // dx, dy はブロック内の座標
                    let pixel_x = x * block_size + dx;
                    let pixel_y = y * block_size + dy;

                    // 元の画像の座標が画像の範囲内であれば、そのピクセルの値を加算する
                    // 例外：ブロックの端のピクセルは、ブロックの範囲外になる場合がある
                    if pixel_x < width && pixel_y < height {
                        let pixel = img.get_pixel(pixel_x, pixel_y);

                        r_sum += pixel[0] as u32;
                        g_sum += pixel[1] as u32;
                        b_sum += pixel[2] as u32;
                        a_sum += pixel[3] as u32;
                    }
                }
            }

            // ブロック内のピクセル数
            let pixel_count = block_size * block_size;

            let r_avg = (r_sum / pixel_count) as u8;
            let g_avg = (g_sum / pixel_count) as u8;
            let b_avg = (b_sum / pixel_count) as u8;
            let a_avg = (a_sum / pixel_count) as u8;

            red_r.push(r_sum % pixel_count);
            green_r.push(g_sum % pixel_count);
            blue_r.push(b_sum % pixel_count);
            alpha_r.push(a_sum % pixel_count);

            // ブロック内のピクセルの平均値
            let avg_pixel = Rgba([r_avg, g_avg, b_avg, a_avg]);

            new_img.put_pixel(x, y, avg_pixel);
        }
    }

    new_img.as_bytes().to_vec()
}

pub fn exec(
    buf: Vec<u8>,
    scale_factor: u32,
    width: u32,
    height: u32,
) -> (
    ImageBuffer<Rgba<u8>, Vec<u8>>,
    Vec<u32>,
    Vec<u32>,
    Vec<u32>,
    Vec<u32>,
) {
    let mut img: ImageBuffer<Rgba<u8>, Vec<_>> = ImageBuffer::from_raw(width, height, buf).unwrap();

    let block_size = scale_factor;

    let new_width = width / block_size;
    let new_height = height / block_size;

    let mut new_img = ImageBuffer::new(new_width, new_height);

    let mut red_r = Vec::new();
    let mut green_r = Vec::new();
    let mut blue_r = Vec::new();
    let mut alpha_r = Vec::new();

    for y in 0..new_height {
        for x in 0..new_width {
            let mut r_sum: u32 = 0;
            let mut g_sum: u32 = 0;
            let mut b_sum: u32 = 0;
            let mut a_sum: u32 = 0;

            for dy in 0..block_size {
                for dx in 0..block_size {
                    // 元の画像の座標
                    // x, y は縮小後の画像(ブロック)の座標
                    // dx, dy はブロック内の座標
                    let pixel_x = x * block_size + dx;
                    let pixel_y = y * block_size + dy;

                    // 元の画像の座標が画像の範囲内であれば、そのピクセルの値を加算する
                    // 例外：ブロックの端のピクセルは、ブロックの範囲外になる場合がある
                    if pixel_x < width && pixel_y < height {
                        let pixel = img.get_pixel(pixel_x, pixel_y);

                        r_sum += pixel[0] as u32;
                        g_sum += pixel[1] as u32;
                        b_sum += pixel[2] as u32;
                        a_sum += pixel[3] as u32;
                    }
                }
            }

            // ブロック内のピクセル数
            let pixel_count = block_size * block_size;

            let r_avg = (r_sum / pixel_count) as u8;
            let g_avg = (g_sum / pixel_count) as u8;
            let b_avg = (b_sum / pixel_count) as u8;
            let a_avg = (a_sum / pixel_count) as u8;

            red_r.push(r_sum % pixel_count);
            green_r.push(g_sum % pixel_count);
            blue_r.push(b_sum % pixel_count);
            alpha_r.push(a_sum % pixel_count);

            // ブロック内のピクセルの平均値
            let avg_pixel = Rgba([r_avg, g_avg, b_avg, a_avg]);

            new_img.put_pixel(x, y, avg_pixel);
        }
    }

    (new_img, red_r, green_r, blue_r, alpha_r)
}

#[derive(Clone, Debug)]
pub struct Config {
    // original picture pixels
    red: Vec<Column<Advice>>,
    green: Vec<Column<Advice>>,
    blue: Vec<Column<Advice>>,
    alpha: Vec<Column<Advice>>,

    // selector for bilinear
    q_bilinear: Selector,

    // reduced pixels
    reduced_red: Vec<Column<Advice>>,
    reduced_green: Vec<Column<Advice>>,
    reduced_blue: Vec<Column<Advice>>,
    reduced_alpha: Vec<Column<Advice>>,

    // instance for bilinear
    instance_red: Vec<Column<Instance>>,
    instance_green: Vec<Column<Instance>>,
    instance_blue: Vec<Column<Instance>>,
    instance_alpha: Vec<Column<Instance>>,

    r_red: Vec<Column<Advice>>,
    r_green: Vec<Column<Advice>>,
    r_blue: Vec<Column<Advice>>,
    r_alpha: Vec<Column<Advice>>,
}

impl Config {
    fn configure<F: FieldExt>(
        cs: &mut ConstraintSystem<F>,
        red: Vec<Column<Advice>>,
        green: Vec<Column<Advice>>,
        blue: Vec<Column<Advice>>,
        alpha: Vec<Column<Advice>>,
        reduced_red: Vec<Column<Advice>>,
        reduced_green: Vec<Column<Advice>>,
        reduced_blue: Vec<Column<Advice>>,
        reduced_alpha: Vec<Column<Advice>>,
        instance_red: Vec<Column<Instance>>,
        instance_green: Vec<Column<Instance>>,
        instance_blue: Vec<Column<Instance>>,
        instance_alpha: Vec<Column<Instance>>,
        r_red: Vec<Column<Advice>>,
        r_green: Vec<Column<Advice>>,
        r_blue: Vec<Column<Advice>>,
        r_alpha: Vec<Column<Advice>>,
    ) -> Self {
        let q_bilinear = cs.selector();

        // ---------------------------------
        // |                |               |
        // |      0         |       1       |
        // |                |               |
        // |                |               |
        // |----------------|---------------|
        // |                |               |
        // |       2        |      3        |
        // |                |               |
        // |                |               |
        // ---------------------------------|

        // 4 X 4のoriginal pictureを1 X 1に縮小する
        // それを全ての列に対して行うgateを作成する
        cs.create_gate("create small pictures", |virtual_cells| {
            // selector on かどうか
            let q_bilinear = virtual_cells.query_selector(q_bilinear);

            let mut bilinear = |range: usize| {
                assert!(range > 0);
                (0..range)
                    .map(|i| {
                        // 4 X 4のoriginal pictureの各ピクセルの値を取得する
                        let red_0 = virtual_cells.query_advice(red[i * 2 + 0], Rotation::cur());
                        let red_1 = virtual_cells.query_advice(red[i * 2 + 1], Rotation::cur());
                        let red_2 = virtual_cells.query_advice(red[i * 2 + 0], Rotation::next());
                        let red_3 = virtual_cells.query_advice(red[i * 2 + 1], Rotation::next());
                        let reduced_red =
                            virtual_cells.query_advice(reduced_red[i], Rotation::cur());
                        let r_red = virtual_cells.query_advice(r_red[i], Rotation::cur());

                        let green_0 = virtual_cells.query_advice(green[i * 2 + 0], Rotation::cur());
                        let green_1 = virtual_cells.query_advice(green[i * 2 + 1], Rotation::cur());
                        let green_2 =
                            virtual_cells.query_advice(green[i * 2 + 0], Rotation::next());
                        let green_3 =
                            virtual_cells.query_advice(green[i * 2 + 1], Rotation::next());
                        let reduced_green =
                            virtual_cells.query_advice(reduced_green[i], Rotation::cur());
                        let r_green = virtual_cells.query_advice(r_green[i], Rotation::cur());

                        let blue_0 = virtual_cells.query_advice(blue[i * 2 + 0], Rotation::cur());
                        let blue_1 = virtual_cells.query_advice(blue[i * 2 + 1], Rotation::cur());
                        let blue_2 = virtual_cells.query_advice(blue[i * 2 + 0], Rotation::next());
                        let blue_3 = virtual_cells.query_advice(blue[i * 2 + 1], Rotation::next());
                        let reduced_blue =
                            virtual_cells.query_advice(reduced_blue[i], Rotation::cur());
                        let r_blue = virtual_cells.query_advice(r_blue[i], Rotation::cur());

                        let alpha_0 = virtual_cells.query_advice(alpha[i * 2 + 0], Rotation::cur());
                        let alpha_1 = virtual_cells.query_advice(alpha[i * 2 + 1], Rotation::cur());
                        let alpha_2 =
                            virtual_cells.query_advice(alpha[i * 2 + 0], Rotation::next());
                        let alpha_3 =
                            virtual_cells.query_advice(alpha[i * 2 + 1], Rotation::next());
                        let reduced_alpha =
                            virtual_cells.query_advice(reduced_alpha[i], Rotation::cur());
                        let r_alpha = virtual_cells.query_advice(r_alpha[i], Rotation::cur());

                        (red_0 + red_1 + red_2 + red_3)
                            - (Expression::Constant(F::from(4)) * reduced_red + r_red)
                            + (green_0 + green_1 + green_2 + green_3)
                            - (Expression::Constant(F::from(4)) * reduced_green + r_green)
                            + (blue_0 + blue_1 + blue_2 + blue_3)
                            - (Expression::Constant(F::from(4)) * reduced_blue + r_blue)
                            + (alpha_0 + alpha_1 + alpha_2 + alpha_3)
                            - (Expression::Constant(F::from(4)) * reduced_alpha + r_alpha)
                    })
                    .collect::<Vec<_>>()
            };

            println!("configure is done");

            Constraints::with_selector(q_bilinear, bilinear(*S_WIDTH.get().unwrap()))
        });

        Self {
            red,
            green,
            blue,
            alpha,
            q_bilinear,
            reduced_red,
            reduced_green,
            reduced_blue,
            reduced_alpha,
            instance_red,
            instance_green,
            instance_blue,
            instance_alpha,
            r_red,
            r_green,
            r_blue,
            r_alpha,
        }
    }

    fn assign<F: FieldExt>(
        &self,
        mut layouter: impl Layouter<F>,

        red: Vec<Value<u8>>,
        green: Vec<Value<u8>>,
        blue: Vec<Value<u8>>,
        alpha: Vec<Value<u8>>,
        reduced_red: Vec<Value<u8>>,
        reduced_green: Vec<Value<u8>>,
        reduced_blue: Vec<Value<u8>>,
        reduced_alpha: Vec<Value<u8>>,
        r_red: Vec<Value<u8>>,
        r_green: Vec<Value<u8>>,
        r_blue: Vec<Value<u8>>,
        r_alpha: Vec<Value<u8>>,
    ) -> Result<
        (
            Vec<AssignedCell<F, F>>,
            Vec<AssignedCell<F, F>>,
            Vec<AssignedCell<F, F>>,
            Vec<AssignedCell<F, F>>,
        ),
        Error,
    > {
        layouter.assign_region(
            || "assign 2 rows",
            |mut region| {
                let mut s_width = 0;

                let mut acc_red = Vec::new();
                let mut acc_green = Vec::new();
                let mut acc_blue = Vec::new();
                let mut acc_alpha = Vec::new();

                for i in (0..*WIDTH.get().unwrap()).step_by(2) {
                    self.q_bilinear.enable(&mut region, 0)?;

                    let red = red
                        .iter()
                        .map(|&v| v.map(|v| F::from(v as u64)))
                        .collect::<Vec<Value<F>>>();
                    let green = green
                        .iter()
                        .map(|&v| v.map(|v| F::from(v as u64)))
                        .collect::<Vec<Value<F>>>();
                    let blue = blue
                        .iter()
                        .map(|&v| v.map(|v| F::from(v as u64)))
                        .collect::<Vec<Value<F>>>();
                    let alpha = alpha
                        .iter()
                        .map(|&v| v.map(|v| F::from(v as u64)))
                        .collect::<Vec<Value<F>>>();

                    let reduced_red = reduced_red
                        .iter()
                        .map(|&v| v.map(|v| F::from(v as u64)))
                        .collect::<Vec<Value<F>>>();
                    let r_red = r_red
                        .iter()
                        .map(|&v| v.map(|v| F::from(v as u64)))
                        .collect::<Vec<Value<F>>>();

                    let reduced_green = reduced_green
                        .iter()
                        .map(|&v| v.map(|v| F::from(v as u64)))
                        .collect::<Vec<Value<F>>>();
                    let r_green = r_green
                        .iter()
                        .map(|&v| v.map(|v| F::from(v as u64)))
                        .collect::<Vec<Value<F>>>();

                    let reduced_blue = reduced_blue
                        .iter()
                        .map(|&v| v.map(|v| F::from(v as u64)))
                        .collect::<Vec<Value<F>>>();
                    let r_blue = r_blue
                        .iter()
                        .map(|&v| v.map(|v| F::from(v as u64)))
                        .collect::<Vec<Value<F>>>();

                    let reduced_alpha = reduced_alpha
                        .iter()
                        .map(|&v| v.map(|v| F::from(v as u64)))
                        .collect::<Vec<Value<F>>>();
                    let r_alpha = r_alpha
                        .iter()
                        .map(|&v| v.map(|v| F::from(v as u64)))
                        .collect::<Vec<Value<F>>>();

                    region.assign_advice(|| "red", self.red[i], 0, || *red.get(i).unwrap())?;
                    region.assign_advice(
                        || "red",
                        self.red[i + 1],
                        0,
                        || *red.get(i + 1).unwrap(),
                    )?;
                    region.assign_advice(
                        || "red",
                        self.red[i],
                        1,
                        || *red.get(i + *WIDTH.get().unwrap()).unwrap(),
                    )?;
                    region.assign_advice(
                        || "red",
                        self.red[i + 1],
                        1,
                        || *red.get(i + *WIDTH.get().unwrap() + 1).unwrap(),
                    )?;

                    region.assign_advice(
                        || "green",
                        self.green[i],
                        0,
                        || *green.get(i).unwrap(),
                    )?;
                    region.assign_advice(
                        || "green",
                        self.green[i + 1],
                        0,
                        || *green.get(i + 1).unwrap(),
                    )?;
                    region.assign_advice(
                        || "green",
                        self.green[i],
                        1,
                        || *green.get(i + *WIDTH.get().unwrap()).unwrap(),
                    )?;
                    region.assign_advice(
                        || "green",
                        self.green[i + 1],
                        1,
                        || *green.get(i + *WIDTH.get().unwrap() + 1).unwrap(),
                    )?;

                    region.assign_advice(|| "blue", self.blue[i], 0, || *blue.get(i).unwrap())?;
                    region.assign_advice(
                        || "blue",
                        self.blue[i + 1],
                        0,
                        || *blue.get(i + 1).unwrap(),
                    )?;
                    region.assign_advice(
                        || "blue",
                        self.blue[i],
                        1,
                        || *blue.get(i + *WIDTH.get().unwrap()).unwrap(),
                    )?;
                    region.assign_advice(
                        || "blue",
                        self.blue[i + 1],
                        1,
                        || *blue.get(i + *WIDTH.get().unwrap() + 1).unwrap(),
                    )?;

                    region.assign_advice(
                        || "alpha",
                        self.alpha[i],
                        0,
                        || *alpha.get(i).unwrap(),
                    )?;
                    region.assign_advice(
                        || "alpha",
                        self.alpha[i + 1],
                        0,
                        || *alpha.get(i + 1).unwrap(),
                    )?;
                    region.assign_advice(
                        || "alpha",
                        self.alpha[i],
                        1,
                        || *alpha.get(i + *WIDTH.get().unwrap()).unwrap(),
                    )?;
                    region.assign_advice(
                        || "alpha",
                        self.alpha[i + 1],
                        1,
                        || *alpha.get(i + *WIDTH.get().unwrap() + 1).unwrap(),
                    )?;

                    region.assign_advice(
                        || "red remainder",
                        self.r_red[s_width],
                        0,
                        || r_red[s_width],
                    )?;
                    region.assign_advice(
                        || "green remainder",
                        self.r_green[s_width],
                        0,
                        || r_green[s_width],
                    )?;
                    region.assign_advice(
                        || "blue remainder",
                        self.r_blue[s_width],
                        0,
                        || r_blue[s_width],
                    )?;
                    region.assign_advice(
                        || "alpha remainder",
                        self.r_alpha[s_width],
                        0,
                        || r_alpha[s_width],
                    )?;

                    let reduced_red = region.assign_advice(
                        || "new_red",
                        self.reduced_red[s_width],
                        0,
                        || reduced_red[s_width],
                    )?;
                    let reduced_green = region.assign_advice(
                        || "new_green",
                        self.reduced_green[s_width],
                        0,
                        || reduced_green[s_width],
                    )?;
                    let reduced_blue = region.assign_advice(
                        || "new_blue",
                        self.reduced_blue[s_width],
                        0,
                        || reduced_blue[s_width],
                    )?;
                    let reduced_alpha = region.assign_advice(
                        || "new_alpha",
                        self.reduced_alpha[s_width],
                        0,
                        || reduced_alpha[s_width],
                    )?;

                    acc_red.push(reduced_red);
                    acc_green.push(reduced_green);
                    acc_blue.push(reduced_blue);
                    acc_alpha.push(reduced_alpha);

                    s_width += 1;
                }
                println!("assign is done");
                Ok((acc_red, acc_green, acc_blue, acc_alpha))
            },
        )
    }

    // fn expose_public<F: FieldExt> (
    //     &self,
    //     mut layouter: impl Layouter<F>,
    //     cell: &AssignedCell<F, F>,
    //     row: usize,
    //     column: usize,
    // ) -> Result<(), Error> {
    //     layouter.constrain_instance(cell.cell(), self.instance_red[column], row)
    // }
}

#[derive(Clone, Debug)]
pub struct MyCircuit<F: FieldExt> {
    pub red: Vec<Value<u8>>,
    pub green: Vec<Value<u8>>,
    pub blue: Vec<Value<u8>>,
    pub alpha: Vec<Value<u8>>,

    pub reduced_red: Vec<Value<u8>>,
    pub reduced_green: Vec<Value<u8>>,
    pub reduced_blue: Vec<Value<u8>>,
    pub reduced_alpha: Vec<Value<u8>>,

    pub r_red: Vec<Value<u8>>,
    pub r_green: Vec<Value<u8>>,
    pub r_blue: Vec<Value<u8>>,
    pub r_alpha: Vec<Value<u8>>,

    pub _marker: PhantomData<F>,
}
// Defaultの実装
impl<F: FieldExt> Default for MyCircuit<F> {
    fn default() -> Self {
        let width = *WIDTH.get().unwrap();
        let height = *HEIGHT.get().unwrap();
        let s_width = *S_WIDTH.get().unwrap();
        let s_height = *S_HEIGHT.get().unwrap();
        Self {
            red: vec![Value::unknown(); width * height],
            green: vec![Value::unknown(); width * height],
            blue: vec![Value::unknown(); width * height],
            alpha: vec![Value::unknown(); width * height],

            reduced_red: vec![Value::unknown(); s_width * s_height],
            reduced_green: vec![Value::unknown(); s_width * s_height],
            reduced_blue: vec![Value::unknown(); s_width * s_height],
            reduced_alpha: vec![Value::unknown(); s_width * s_height],
        

            r_red: vec![Value::unknown(); s_width * s_height],
            r_green: vec![Value::unknown(); s_width * s_height],
            r_blue: vec![Value::unknown(); s_width * s_height],
            r_alpha: vec![Value::unknown(); s_width * s_height],

            _marker: PhantomData,
        }
    }
}

impl<F: FieldExt> Circuit<F> for MyCircuit<F> {
    type Config = Config;

    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
        let width = *WIDTH.get().unwrap();
        let s_width = *S_WIDTH.get().unwrap();
        // columnの設定
        let red = (0..width)
            .map(|_| meta.advice_column())
            .collect::<Vec<_>>();
        let green = (0..width)
            .map(|_| meta.advice_column())
            .collect::<Vec<_>>();
        let blue = (0..width)
            .map(|_| meta.advice_column())
            .collect::<Vec<_>>();
        let alpha = (0..width)
            .map(|_| meta.advice_column())
            .collect::<Vec<_>>();

        // columnの設定 & equalityの設定
        let reduced_red = (0..s_width)
            .map(|_| meta.advice_column())
            .collect::<Vec<_>>();
        reduced_red
            .iter()
            .for_each(|&col| meta.enable_equality(col));
        let reduced_green = (0..s_width)
            .map(|_| meta.advice_column())
            .collect::<Vec<_>>();
        reduced_green
            .iter()
            .for_each(|&col| meta.enable_equality(col));
        let reduced_blue = (0..s_width)
            .map(|_| meta.advice_column())
            .collect::<Vec<_>>();
        reduced_blue
            .iter()
            .for_each(|&col| meta.enable_equality(col));
        let reduced_alpha = (0..s_width)
            .map(|_| meta.advice_column())
            .collect::<Vec<_>>();
        reduced_alpha
            .iter()
            .for_each(|&col| meta.enable_equality(col));

        let instance_red = (0..s_width)
            .map(|_| meta.instance_column())
            .collect::<Vec<_>>();
        instance_red
            .iter()
            .for_each(|&col| meta.enable_equality(col));
        let instance_green = (0..s_width)
            .map(|_| meta.instance_column())
            .collect::<Vec<_>>();
        instance_green
            .iter()
            .for_each(|&col| meta.enable_equality(col));
        let instance_blue = (0..s_width)
            .map(|_| meta.instance_column())
            .collect::<Vec<_>>();
        instance_blue
            .iter()
            .for_each(|&col| meta.enable_equality(col));
        let instance_alpha = (0..s_width)
            .map(|_| meta.instance_column())
            .collect::<Vec<_>>();
        instance_alpha
            .iter()
            .for_each(|&col| meta.enable_equality(col));

        let r_red = (0..s_width)
            .map(|_| meta.advice_column())
            .collect::<Vec<_>>();
        let r_green = (0..s_width)
            .map(|_| meta.advice_column())
            .collect::<Vec<_>>();
        let r_blue = (0..s_width)
            .map(|_| meta.advice_column())
            .collect::<Vec<_>>();
        let r_alpha = (0..s_width)
            .map(|_| meta.advice_column())
            .collect::<Vec<_>>();

        Self::Config::configure(
            meta,
            red.try_into().unwrap(),
            green.try_into().unwrap(),
            blue.try_into().unwrap(),
            alpha.try_into().unwrap(),
            reduced_red.try_into().unwrap(),
            reduced_green.try_into().unwrap(),
            reduced_blue.try_into().unwrap(),
            reduced_alpha.try_into().unwrap(),
            instance_red.try_into().unwrap(),
            instance_green.try_into().unwrap(),
            instance_blue.try_into().unwrap(),
            instance_alpha.try_into().unwrap(),
            r_red.try_into().unwrap(),
            r_green.try_into().unwrap(),
            r_blue.try_into().unwrap(),
            r_alpha.try_into().unwrap(),
        )
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<F>,
    ) -> Result<(), Error> {
        let width = *WIDTH.get().unwrap();
        let s_width = *S_WIDTH.get().unwrap();
        // assignの呼び出し
        // pixelを２行ずつ渡し、1 X S_WIDTHのpixelを受け取る
        let mut index = 0;
        let mut s_index = 0;
        for i in (0..*HEIGHT.get().unwrap()).step_by(2) {
            if *HEIGHT.get().unwrap() % 2 == 0 {
                // let (s_red,s_green, s_blue, s_alpha)
                // = config.assign(layouter.namespace(|| "next row (even)"), [self.red[i], self.red[i + 1]], [self.green[i], self.green[i + 1]], [self.blue[i], self.blue[i + 1]],[self.alpha[i], self.alpha[i + 1]], self.reduced_red[i / 2], self.reduced_green[i / 2], self.reduced_blue[i / 2], self.reduced_alpha[i / 2], self.r_red[i / 2], self.r_green[i / 2], self.r_blue[i / 2], self.r_alpha[i / 2], self.value)?;

                let (s_red, s_green, s_blue, s_alpha) = config.assign(
                    layouter.namespace(|| "next row (even)"),
                    self.red[index..(index + 2 * width)].to_vec(),
                    self.green[index..(index + 2 * width)].to_vec(),
                    self.blue[index..(index + 2 * width)].to_vec(),
                    self.alpha[index..(index + 2 * width)].to_vec(),
                    self.reduced_red[s_index..(s_index + s_width)].to_vec(),
                    self.reduced_green[s_index..(s_index + s_width)].to_vec(),
                    self.reduced_blue[s_index..(s_index + s_width)].to_vec(),
                    self.reduced_alpha[s_index..(s_index + s_width)].to_vec(),
                    self.r_red[s_index..(s_index + s_width)].to_vec(),
                    self.r_green[s_index..(s_index + s_width)].to_vec(),
                    self.r_blue[s_index..(s_index + s_width)].to_vec(),
                    self.r_alpha[s_index..(s_index + s_width)].to_vec(),
                )?;

                for column in 0..*S_WIDTH.get().unwrap() {
                    layouter.constrain_instance(
                        s_red[column].cell(),
                        config.instance_red[column],
                        i / 2,
                    )?;
                    layouter.constrain_instance(
                        s_green[column].cell(),
                        config.instance_green[column],
                        i / 2,
                    )?;
                    layouter.constrain_instance(
                        s_blue[column].cell(),
                        config.instance_blue[column],
                        i / 2,
                    )?;
                    layouter.constrain_instance(
                        s_alpha[column].cell(),
                        config.instance_alpha[column],
                        i / 2,
                    )?;
                }
            } else {
                if i == (*HEIGHT.get().unwrap() - 1) {
                    continue;
                } else {
                    let (s_red, s_green, s_blue, s_alpha) = config.assign(
                        layouter.namespace(|| "next row (even)"),
                        self.red[index..(index + 2 * width)].to_vec(),
                        self.green[index..(index + 2 * width)].to_vec(),
                        self.blue[index..(index + 2 * width)].to_vec(),
                        self.alpha[index..(index + 2 * width)].to_vec(),
                        self.reduced_red[s_index..(s_index + s_width)].to_vec(),
                        self.reduced_green[s_index..(s_index + s_width)].to_vec(),
                        self.reduced_blue[s_index..(s_index + s_width)].to_vec(),
                        self.reduced_alpha[s_index..(s_index + s_width)].to_vec(),
                        self.r_red[s_index..(s_index + s_width)].to_vec(),
                        self.r_green[s_index..(s_index + s_width)].to_vec(),
                        self.r_blue[s_index..(s_index + s_width)].to_vec(),
                        self.r_alpha[s_index..(s_index + s_width)].to_vec(),
                    )?;

                    for column in 0..s_width {
                        layouter.constrain_instance(
                            s_red[column].cell(),
                            config.instance_red[column],
                            i / 2,
                        )?;
                        layouter.constrain_instance(
                            s_green[column].cell(),
                            config.instance_green[column],
                            i / 2,
                        )?;
                        layouter.constrain_instance(
                            s_blue[column].cell(),
                            config.instance_blue[column],
                            i / 2,
                        )?;
                        layouter.constrain_instance(
                            s_alpha[column].cell(),
                            config.instance_alpha[column],
                            i / 2,
                        )?;
                    }
                }
            }
            index += 2 * width;
            s_index += s_width;
        }
        println!("synthesize now");
        Ok(())
    }
}

pub fn create_img_proof(origin_buf: Vec<u8>, width: u32, height: u32) -> Vec<u8> {

    let mut img: ImageBuffer<Rgba<u8>, Vec<_>> =
        ImageBuffer::from_raw(width, height, origin_buf.clone()).unwrap();
    let width = img.width();
    let height = img.height();
    let scale_factor = 2; // 縮小率

    let (result, r_red, r_green, r_blue, r_alpha) = exec(origin_buf, scale_factor, width, height);
    let r_red = r_red
        .iter()
        .map(|&v| Value::known(v as u8))
        .collect::<Vec<Value<u8>>>();
    let r_green = r_green
        .iter()
        .map(|&v| Value::known(v as u8))
        .collect::<Vec<Value<u8>>>();
    let r_blue = r_blue
        .iter()
        .map(|&v| Value::known(v as u8))
        .collect::<Vec<Value<u8>>>();
    let r_alpha = r_alpha
        .iter()
        .map(|&v| Value::known(v as u8))
        .collect::<Vec<Value<u8>>>();

    let s_width = result.width();
    let s_height = result.height();

    let _ = WIDTH.set(width as usize);
    let _ = HEIGHT.set(height as usize);
    let _ = S_WIDTH.set(s_width as usize);
    let _ = S_HEIGHT.set(s_height as usize);
    let param = &*PARAMETER;
    let params = param.get_parameter();

    let mut red = Vec::new();
    let mut blue = Vec::new();
    let mut green = Vec::new();
    let mut alpha = Vec::new();

    let mut reduced_red = Vec::new();
    let mut reduced_blue = Vec::new();
    let mut reduced_green = Vec::new();
    let mut reduced_alpha = Vec::new();

    println!("start");

    for i in 0..height as u32 {
        for j in 0..width as u32 {
            let pixel = img.get_pixel(j, i);

            red.push(Value::known(pixel[0]));
            green.push(Value::known(pixel[1]));
            blue.push(Value::known(pixel[2]));
            alpha.push(Value::known(pixel[3]));
        }
    }

    println!("got pixels");

    for i in 0..s_height {
        for j in 0..s_width {
            let pixel = result.get_pixel(j, i);
            reduced_red.push(Value::known(pixel[0]));
            reduced_green.push(Value::known(pixel[1]));
            reduced_blue.push(Value::known(pixel[2]));
            reduced_alpha.push(Value::known(pixel[3]));
        }
    }
    let empty_circuit = MyCircuit::<Fp>::default();
    // println!("empty circuit{:?}", empty_circuit);

    println!("red: {:?}", red.len());
    let circuit = MyCircuit::<Fp> {
        red,
        green,
        blue,
        alpha,
        reduced_red,
        reduced_green,
        reduced_blue,
        reduced_alpha,
        r_red,
        r_green,
        r_blue,
        r_alpha,
        _marker: PhantomData,
    };

    let mut instance_red = Vec::new();
    let mut instance_blue = Vec::new();
    let mut instance_green = Vec::new();
    let mut instance_alpha = Vec::new();

    for i in 0..s_width {
        for j in 0..s_height {
            let pixel = result.get_pixel(i, j);
            instance_red.push(Fp::from(pixel[0] as u64));
            instance_green.push(Fp::from(pixel[1] as u64));
            instance_blue.push(Fp::from(pixel[2] as u64));
            instance_alpha.push(Fp::from(pixel[3] as u64));
        }
    }

    // let instance = Fp::from(55);
    // let mut public_input = vec![instance];
    let mut public_input = Vec::new();

    let mut index = 0;
    for _ in 0..s_width {
        public_input.push(&instance_red[index..(index + *S_HEIGHT.get().unwrap())]);
        index += *S_HEIGHT.get().unwrap();
    }
    index = 0;
    for _ in 0..s_width {
        public_input.push(&instance_green[index..(index + *S_HEIGHT.get().unwrap())]);
        index += *S_HEIGHT.get().unwrap();
    }
    index = 0;
    for _ in 0..s_width {
        public_input.push(&instance_blue[index..(index + *S_HEIGHT.get().unwrap())]);
        index += *S_HEIGHT.get().unwrap();
    }
    index = 0;
    for _ in 0..s_width {
        public_input.push(&instance_alpha[index..(index + *S_HEIGHT.get().unwrap())]);
        index += *S_HEIGHT.get().unwrap();
    }

    // let params_fs = File::open("params.bin").unwrap();
    // let params = Params::<EqAffine>::read(&mut BufReader::new(params_fs)).unwrap();
    // let params = Params::<EqAffine>::read(&mut BufReader::new(&params_vec[..])).unwrap();
    let vk = keygen_vk(&params, &empty_circuit).expect("keygen_vk should not fail");
    let pk = keygen_pk(&params, vk.clone(), &empty_circuit).expect("keygen_pk should not fail");
    println!("Successfully generated proving key");

    let mut transcript = Blake2bWrite::<_, _, Challenge255<_>>::init(vec![]);
    // Create a proof
    halo2_proofs::plonk::create_proof(
        &params,
        &pk,
        &[circuit.clone()],
        &[&public_input[..]],
        OsRng,
        &mut transcript,
    )
    .expect("proof generation should not fail");
    let proof: Vec<u8> = transcript.finalize();
    proof
    
}

pub fn verify_img(proof: Vec<u8>, small_buf: Vec<u8>, s_width: u32, s_height: u32) -> bool {


    let mut img: ImageBuffer<Rgba<u8>, Vec<_>> =
        ImageBuffer::from_raw(s_width, s_height, small_buf).unwrap();

    let empty_circuit = MyCircuit::<Fp>::default();

    let mut public_input = Vec::new();

    let mut instance_red = Vec::new();
    let mut instance_blue = Vec::new();
    let mut instance_green = Vec::new();
    let mut instance_alpha = Vec::new();

    for i in 0..s_width {
        for j in 0..s_height {
            let pixel = img.get_pixel(i, j);
            instance_red.push(Fp::from(pixel[0] as u64));
            instance_green.push(Fp::from(pixel[1] as u64));
            instance_blue.push(Fp::from(pixel[2] as u64));
            instance_alpha.push(Fp::from(pixel[3] as u64));
        }
    }

    let mut index = 0;
    for _ in 0..s_width {
        public_input.push(&instance_red[index..(index + *S_HEIGHT.get().unwrap())]);
        index += *S_HEIGHT.get().unwrap();
    }
    index = 0;
    for _ in 0..s_width {
        public_input.push(&instance_green[index..(index + *S_HEIGHT.get().unwrap())]);
        index += *S_HEIGHT.get().unwrap();
    }
    index = 0;
    for _ in 0..s_width {
        public_input.push(&instance_blue[index..(index + *S_HEIGHT.get().unwrap())]);
        index += *S_HEIGHT.get().unwrap();
    }
    index = 0;
    for _ in 0..s_width {
        public_input.push(&instance_alpha[index..(index + *S_HEIGHT.get().unwrap())]);
        index += *S_HEIGHT.get().unwrap();
    }

    // let params_fs = File::open("params.bin").unwrap();
    // let params = Params::<EqAffine>::read(&mut BufReader::new(params_fs)).unwrap();
    // let params = Params::<EqAffine>::read(&mut BufReader::new(&params_vec[..])).unwrap();
    let param = &*PARAMETER;
    let params = param.get_parameter();

    // let proof = proof_js.into_serde::<Vec<u8>>().unwrap();

    let vk = keygen_vk(&params, &empty_circuit).expect("keygen_vk should not fail");

    // Check that a hardcoded proof is satisfied
    let strategy = SingleVerifier::new(&params);
    let mut transcript = Blake2bRead::<_, _, Challenge255<_>>::init(&proof[..]);

    verify_proof(
        &params,
        &vk,
        strategy,
        &[&public_input[..]],
        &mut transcript,
    )
    .is_ok()
}
