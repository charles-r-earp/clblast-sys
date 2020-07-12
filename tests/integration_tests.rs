extern crate clblast_sys;
use clblast_sys::{CLBlastStatusCode, CLBlastSgemm, CLBlastLayout::*, CLBlastTranspose::*, cl_mem, cl_command_queue};
use ocl::{Context, Queue, Buffer};
use ndarray::{Array, Array2, ArrayView2};
use num_traits::ToPrimitive;

/*
fn compare_vectors(a: &[f32], b: &[f32], m: usize, k: usize, n: usize) {
    // CLBlast has fast mul / approx ops
    // compared to ndarray / matrixmultiply which performs exact ops
    // assert_eq fails for gemm with large matrices
    // https://oneapi-src.github.io/oneDNN/cpu_sgemm_and_matmul_8cpp-example.html
    let mut v1_l2 = 0f64;
    let mut diff_l2 = 0f64;
    a.iter().zip(b.iter()).for_each(|(&a, &b)| {
        v1_l2 += (a * b) as f64;
        diff_l2 += (a - b).powi(2) as f64;
    });
    let threshold = (f32::EPSILON as f64) * f64::ln(f64::max(2., k.to_f64().unwrap()));
    assert!(
        diff_l2.sqrt() <= threshold * v1_l2.sqrt(),
        "m: {} k: {} n: {} ({} !<= {})",
        m,
        k,
        n,
        diff_l2.sqrt(),
        threshold * v1_l2.sqrt()
    );
}*/

fn matmul(lhs: &ArrayView2<f32>, rhs: &ArrayView2<f32>) -> Array2<f32> {
    let context = Context::builder()
        .build()
        .unwrap();
    let device = context.devices()[0];
    let queue = Queue::new(&context, device, None)
        .unwrap();
    
    let (m, k) = lhs.dim();
    let (_, n) = rhs.dim();

    let x1_buf = Buffer::<f32>::builder()
        .queue(queue.clone())
        .len(m * k)
        .copy_host_slice(&lhs.as_slice().unwrap())
        .build()
        .unwrap();
    let x2_buf = Buffer::<f32>::builder()
        .queue(queue.clone())
        .len(k * n)
        .copy_host_slice(&rhs.as_slice().unwrap())
        .build()
        .unwrap();
    
    let y_buf = Buffer::<f32>::builder()
        .queue(queue.clone())
        .len(m * n)
        .build()
        .unwrap();
    
    let alpha = 1f32;
    let beta = 0f32;
    
    let mut command_queue = queue.as_ptr();
    let command_queue = unsafe { &mut command_queue as *mut *mut std::ffi::c_void }; 
    
    let status = unsafe { 
        CLBlastSgemm( 
            CLBlastLayoutRowMajor,
            CLBlastTransposeNo,
            CLBlastTransposeNo,
            m, n, k,
            alpha,
            x1_buf.as_ptr() as cl_mem,
            0, k,
            x2_buf.as_ptr() as cl_mem,
            0, n,
            beta,
            y_buf.as_ptr() as cl_mem,
            0, n,
            command_queue as *mut cl_command_queue,
            std::ptr::null_mut()
        )
    };
    
    assert_eq!(status, CLBlastStatusCode::CLBlastSuccess);
    
    let mut output = Array2::zeros([m, n]);
    
    y_buf.read(output.as_slice_mut().unwrap())
        .enq()
        .unwrap();

    output
}

#[test]
fn test_matmul() {
    let m = 33;
    let k = 43;
    let n = 53;
    
    let vec1: Vec<f32> = (1..=m * k)
        .into_iter()
        .map(|x| x.to_f32().unwrap())
        .collect();
    let vec2: Vec<f32> = (1..=k * n)
        .into_iter()
        .map(|x| x.to_f32().unwrap())
        .collect();
 
    let x1 = Array::from_shape_vec([m, k], vec1).unwrap();
    let x2 = Array::from_shape_vec([k, n], vec2).unwrap();
    
    let y_cl = matmul(&x1.view(), &x2.view());
    let y_arr = x1.dot(&x2);
    
    assert_eq!(y_cl, y_arr);
    //compare_vectors(y_cl.as_slice().unwrap(), y_arr.as_slice().unwrap(), m, k, n);
}
