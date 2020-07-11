extern crate clblast_sys;
use clblast_sys::{CLBlastStatusCode, CLBlastSaxpy, cl_mem, cl_command_queue};
use ocl::{Device, Context, Queue, Buffer, Program, Kernel};

fn main() -> ocl::Result<()> {

    let context = Context::builder()
        .build()?;
    let device = context.devices()[0];
    let queue = Queue::new(&context, device, None)?;
    
    let x_vec = vec![1., 2., 3., 4.];
        
    let n = x_vec.len();

    let x_buf = Buffer::<f32>::builder()
        .queue(queue.clone())
        .len(n)
        .copy_host_slice(&x_vec)
        .build()?;
    
    let y_buf = Buffer::<f32>::builder()
        .queue(queue.clone())
        .len(n)
        .build()?;
    
    let alpha: f32 = 1.0;
    let offx = 0;
    let incx = 1;
    let offy = 0;
    let incy = 1;
    
    let mut command_queue = queue.as_ptr();
    let command_queue = unsafe { &mut command_queue as *mut *mut std::ffi::c_void }; 
    
    let status = unsafe { 
        CLBlastSaxpy( 
            n,
            alpha,
            x_buf.as_ptr() as cl_mem,
            offx,
            incx,
            y_buf.as_ptr() as cl_mem,
            offy,
            incy,
            command_queue as *mut cl_command_queue,
            std::ptr::null_mut()
        )
    };
    
    assert_eq!(status, CLBlastStatusCode::CLBlastSuccess);
    
    
    let mut y_vec = vec![0.0f32; n];
    y_buf.read(&mut y_vec).enq()?;

    println!("{:?}", y_vec);
    Ok(())
}
