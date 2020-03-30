use core::ptr;
use h264_decoder_rs::*;
use std::fs::File;
use std::io::Read;

fn main() -> Result<(), std::io::Error> {
    println!("Loading file");

    let mut file = File::open("h264_decoder/data/video_full.avi")?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    println!("Read {} bytes", data.len());

    let mut decoder: H264SwDecInst = ptr::null_mut();

    let no_output_reorder = 1;
    let err = unsafe { H264SwDecInit(&mut decoder as *mut _, no_output_reorder) };
    assert_eq!(err, 0);

    println!("H264SwDecInit ok");

    // TODO - check u32 vs u8 buffers/pointers

    // TODO - does input really need to be mut?
    let mut input = H264SwDecInput {
        pStream: data.as_mut_ptr(),
        dataLen: data.len() as _,
        picId: 0,
        intraConcealmentMethod: 0,
    };

    println!("input: {:#?}", input);

    let mut output = H264SwDecOutput {
        pStrmCurrPos: ptr::null_mut(),
    };

    println!("output: {:#?}", output);

    println!("Tail: 0x{:X}", data.as_ptr() as usize + data.len());

    input.picId += 1;

    loop {
        println!("Decode");
        let err = unsafe { H264SwDecDecode(decoder, &mut input as *mut _, &mut output as *mut _) };

        println!("error: {}", err);
        println!("input: {:?}", input);
        println!("output: {:?}", output);

        match err {
            H264SwDecRet_H264SWDEC_PIC_RDY => {
                println!("Got pic");
                break;
            }
            H264SwDecRet_H264SWDEC_PIC_RDY_BUFF_NOT_EMPTY => {
                println!("Got pic, not-empty buf");
                break;
            }
            H264SwDecRet_H264SWDEC_HDRS_RDY_BUFF_NOT_EMPTY => {
                println!("Found headers");

                let mut info = H264SwDecInfo {
                    profile: 0,
                    picWidth: 0,
                    picHeight: 0,
                    videoRange: 0,
                    matrixCoefficients: 0,
                    parWidth: 0,
                    parHeight: 0,
                    croppingFlag: 0,
                    cropParams: CropParams {
                        cropLeftOffset: 0,
                        cropOutWidth: 0,
                        cropTopOffset: 0,
                        cropOutHeight: 0,
                    },
                };
                let err = unsafe { H264SwDecGetInfo(decoder, &mut info as *mut _) };
                assert_eq!(err, 0);
                println!("{:#?}", info);
            }
            _ => panic!(),
        }
    }

    println!("Get picture data");

    let mut pic = H264SwDecPicture {
        pOutputPicture: ptr::null_mut(),
        picId: 0,
        isIdrPicture: 0,
        nbrOfErrMBs: 0,
    };

    let err = unsafe { H264SwDecNextPicture(decoder, &mut pic as *mut _, 1) };
    assert_eq!(err, 2);

    println!("{:#?}", pic);

    unsafe { H264SwDecRelease(decoder) };

    Ok(())
}
