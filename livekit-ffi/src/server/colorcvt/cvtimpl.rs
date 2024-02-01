use super::*;
use crate::proto;
use imgproc::colorcvt;

pub fn cvt_rgba(
    buffer: proto::VideoBufferInfo,
    dst: proto::VideoBufferType,
    flip_y: bool,
) -> FfiResult<(Box<[u8]>, proto::VideoBufferInfo)> {
    assert_eq!(buffer.r#type(), proto::VideoBufferType::Rgba);
    let proto::VideoBufferInfo { stride, width, height, data_ptr, data_len, .. } = buffer;
    let data = unsafe { slice::from_raw_parts(data_ptr as *const u8, data_len as usize) };

    match dst {
        proto::VideoBufferType::I420 => {
            let chroma_w = (width + 1) / 2;
            let chroma_h = (height + 1) / 2;
            let mut dst_i420 = vec![0u8; (width * height + chroma_w * chroma_h * 2) as usize];
            let (dy, du, dv) =
                split_i420_mut(dst_i420.as_mut_slice(), width, chroma_w, chroma_w, height);

            colorcvt::abgr_to_i420(
                data, stride, dy, width, du, chroma_w, dv, chroma_w, width, height, flip_y,
            );

            Ok((dst_i420.into_boxed_slice(), i420_info(dst_i420.as_slice(), width, height)))
        }
        _ => Err(FfiError::InvalidRequest(format!("rgba to {:?} is not supported", dst).into())),
    }
}

pub fn cvt_abgr(
    buffer: proto::VideoBufferInfo,
    dst_type: proto::VideoBufferType,
    flip_y: bool,
) -> FfiResult<(Box<[u8]>, proto::VideoBufferInfo)> {
    assert_eq!(buffer.r#type(), proto::VideoBufferType::Rgba);
    let proto::VideoBufferInfo { stride, width, height, data_ptr, data_len, .. } = buffer;
    let data = unsafe { slice::from_raw_parts(data_ptr as *const u8, data_len as usize) };

    match dst_type {
        proto::VideoBufferType::I420 => {
            let chroma_w = (width + 1) / 2;
            let chroma_h = (height + 1) / 2;
            let mut dst_i420 =
                vec![0u8; (width * height + chroma_w * chroma_h * 2) as usize].into_boxed_slice();
            let (dst_y, dst_u, dst_v) =
                split_i420_mut(&mut dst_i420, width, chroma_w, chroma_w, height);

            imgproc::colorcvt::rgba_to_i420(
                data, stride, dst_y, width, dst_u, chroma_w, dst_v, chroma_w, width, height, flip_y,
            );

            Ok((dst_i420, i420_info(&dst_i420, width, height)))
        }
        _ => {
            Err(FfiError::InvalidRequest(format!("abgr to {:?} is not supported", dst_type).into()))
        }
    }
}

pub fn cvt_argb(
    buffer: proto::VideoBufferInfo,
    dst: proto::VideoBufferType,
    flip_y: bool,
) -> FfiResult<(Box<[u8]>, proto::VideoBufferInfo)> {
    assert_eq!(buffer.r#type(), proto::VideoBufferType::Argb);
    let proto::VideoBufferInfo { stride, width, height, data_ptr, data_len, .. } = buffer;
    let data = unsafe { slice::from_raw_parts(data_ptr as *const u8, data_len as usize) };

    match dst {
        proto::VideoBufferType::I420 => {
            let chroma_w = (width + 1) / 2;
            let chroma_h = (height + 1) / 2;
            let mut dst_i420 =
                vec![0u8; (width * height + chroma_w * chroma_h * 2) as usize].into_boxed_slice();
            let (dst_y, dst_u, dst_v) =
                split_i420_mut(&mut dst_i420, width, chroma_w, chroma_w, height);

            colorcvt::bgra_to_i420(
                data, stride, dst_y, width, dst_u, chroma_w, dst_v, chroma_w, width, height, flip_y,
            );

            Ok((dst_i420, i420_info(&dst_i420, width, height)))
        }
        _ => Err(FfiError::InvalidRequest(format!("argb to {:?} is not supported", dst).into())),
    }
}

pub fn cvt_bgra(
    buffer: proto::VideoBufferInfo,
    dst: proto::VideoBufferType,
    flip_y: bool,
) -> FfiResult<(Box<[u8]>, proto::VideoBufferInfo)> {
    assert_eq!(buffer.r#type(), proto::VideoBufferType::Bgra);
    let proto::VideoBufferInfo { stride, width, height, data_ptr, data_len, .. } = buffer;
    let data = unsafe { slice::from_raw_parts(data_ptr as *const u8, data_len as usize) };

    match dst {
        proto::VideoBufferType::I420 => {
            let chroma_w = (width + 1) / 2;
            let chroma_h = (height + 1) / 2;
            let mut dst_i420 = vec![0u8; (width * height + chroma_w * chroma_h * 2) as usize];
            let (dst_y, dst_u, dst_v) =
                split_i420_mut(dst_i420.as_mut_slice(), width, chroma_w, chroma_w, height);

            colorcvt::argb_to_i420(
                data, stride, dst_y, width, dst_u, chroma_w, dst_v, chroma_w, width, height, flip_y,
            );

            Ok((dst_i420.into_boxed_slice(), i420_info(dst_i420.as_slice(), width, height)))
        }
        _ => Err(FfiError::InvalidRequest(format!("bgra to {:?} is not supported", dst).into())),
    }
}

pub fn cvt_rgb24(
    buffer: proto::VideoBufferInfo,
    dst_type: proto::VideoBufferType,
    flip_y: bool,
) -> FfiResult<(Box<[u8]>, proto::VideoBufferInfo)> {
    assert_eq!(buffer.r#type(), proto::VideoBufferType::Rgb24);
    let proto::VideoBufferInfo { stride, width, height, data_ptr, data_len, .. } = buffer;
    let data = unsafe { slice::from_raw_parts(data_ptr as *const u8, data_len as usize) };

    match dst_type {
        proto::VideoBufferType::I420 => {
            let chroma_w = (width + 1) / 2;
            let chroma_h = (height + 1) / 2;
            let mut dst =
                vec![0u8; (width * height + chroma_w * chroma_h * 2) as usize].into_boxed_slice();
            let (dst_y, dst_u, dst_v) = split_i420_mut(&mut dst, width, chroma_w, chroma_w, height);

            colorcvt::raw_to_i420(
                data, stride, dst_y, width, dst_u, chroma_w, dst_v, chroma_w, width, height, flip_y,
            );

            Ok((dst, i420_info(&dst, width, height)))
        }
        _ => {
            return Err(FfiError::InvalidRequest(
                format!("rgb24 to {:?} is not supported", dst_type).into(),
            ))
        }
    }
}

pub fn cvt_i420(
    buffer: proto::VideoBufferInfo,
    dst_type: proto::VideoBufferType,
    flip_y: bool,
) -> FfiResult<(Box<[u8]>, proto::VideoBufferInfo)> {
    assert_eq!(buffer.r#type(), proto::VideoBufferType::I420);
    let proto::VideoBufferInfo { width, height, components, data_ptr, data_len, .. } = buffer;
    let data = unsafe { slice::from_raw_parts(data_ptr as *const u8, data_len as usize) };

    let [c0, c1, c2, ..] = components.as_slice();
    let (y, u, v) = split_i420(data, c0.stride, c1.stride, c2.stride, height);

    match dst_type {
        proto::VideoBufferType::Rgba
        | proto::VideoBufferType::Abgr
        | proto::VideoBufferType::Argb
        | proto::VideoBufferType::Bgra => {
            let mut dst = vec![0u8; (width * height * 4) as usize].into_boxed_slice();
            let stride = width * 4;

            macro_rules! cvt {
                ($rgba:expr, $fnc:ident) => {
                    if dst_type == $rgba {
                        colorcvt::$fnc(
                            y, c0.stride, u, c1.stride, v, c2.stride, &mut dst, stride, width,
                            height, flip_y,
                        );
                    }
                };
            }

            cvt!(proto::VideoBufferType::Rgba, i420_to_abgr);
            cvt!(proto::VideoBufferType::Abgr, i420_to_rgba);
            cvt!(proto::VideoBufferType::Argb, i420_to_bgra);
            cvt!(proto::VideoBufferType::Bgra, i420_to_argb);

            Ok((dst, rgba_info(&dst, dst_type, width, height)))
        }
        proto::VideoBufferType::Rgb24 => {
            let mut dst = vec![0u8; (width * height * 3) as usize];
            let stride = width * 3;

            colorcvt::i420_to_raw(
                y, c0.stride, u, c1.stride, v, c2.stride, &mut dst, stride, width, height, flip_y,
            );

            Ok((dst.into_boxed_slice(), rgb_info(dst.as_slice(), dst_type, width, height)))
        }
        proto::VideoBufferType::I420 => {
            let chroma_w = (width + 1) / 2;
            let chroma_h = (height + 1) / 2;
            let mut dst =
                vec![0u8; (width * height + chroma_w * chroma_h * 2) as usize].into_boxed_slice();
            let (dst_y, dst_u, dst_v) = split_i420_mut(&mut dst, width, chroma_w, chroma_w, height);

            colorcvt::i420_copy(
                y, c0.stride, u, c1.stride, v, c2.stride, dst_y, width, dst_u, chroma_w, dst_v,
                chroma_w, width, height, flip_y,
            );

            Ok((dst, i420_info(&dst, width, height)))
        }
        _ => {
            return Err(FfiError::InvalidRequest(
                format!("i420 to {:?} is not supported", dst_type).into(),
            ))
        }
    }
}

pub fn cvt_i420a(
    buffer: proto::VideoBufferInfo,
    dst_type: proto::VideoBufferType,
    flip_y: bool,
) -> FfiResult<(Box<[u8]>, proto::VideoBufferInfo)> {
    assert_eq!(buffer.r#type(), proto::VideoBufferType::I420a);
    let proto::VideoBufferInfo { width, height, components, data_ptr, data_len, .. } = buffer;
    let data = unsafe { slice::from_raw_parts(data_ptr as *const u8, data_len as usize) };

    let [c0, c1, c2, c3, ..] = components.as_slice();
    let (y, u, v, a) = split_i420a(data, c0.stride, c1.stride, c2.stride, c3.stride, height);

    match dst_type {
        proto::VideoBufferType::I420 => {
            let chroma_w = (width + 1) / 2;
            let chroma_h = (height + 1) / 2;
            let mut dst =
                vec![0u8; (width * height + chroma_w * chroma_h * 2) as usize].into_boxed_slice();
            let (dst_y, dst_u, dst_v) = split_i420_mut(&mut dst, width, chroma_w, chroma_w, height);

            colorcvt::i420_copy(
                y, c0.stride, u, c1.stride, v, c2.stride, dst_y, width, dst_u, chroma_w, dst_v,
                chroma_w, width, height, flip_y,
            );
            Ok((dst, i420_info(&dst, width, height)))
        }
        _ => {
            return Err(FfiError::InvalidRequest(
                format!("i420a to {:?} is not supported", dst_type).into(),
            ))
        }
    }
}

pub fn cvt_i422(
    buffer: proto::VideoBufferInfo,
    dst_type: proto::VideoBufferType,
    flip_y: bool,
) -> FfiResult<(Box<[u8]>, proto::VideoBufferInfo)> {
    assert_eq!(buffer.r#type(), proto::VideoBufferType::I422);
    let proto::VideoBufferInfo { width, height, components, data_ptr, data_len, .. } = buffer;
    let data = unsafe { slice::from_raw_parts(data_ptr as *const u8, data_len as usize) };

    let [c0, c1, c2, ..] = components.as_slice();
    let (y, u, v) = split_i422(data, c0.stride, c1.stride, c2.stride, height);

    match dst_type {
        proto::VideoBufferType::Rgba
        | proto::VideoBufferType::Abgr
        | proto::VideoBufferType::Argb => {
            let mut dst = vec![0u8; (buffer.width * buffer.height * 4) as usize].into_boxed_slice();
            let stride = buffer.width * 4;

            macro_rules! cvt {
                ($rgba:expr, $fnc:ident) => {
                    if dst_type == $rgba {
                        colorcvt::$fnc(
                            y, c0.stride, u, c1.stride, v, c2.stride, &mut dst, stride, width,
                            height, flip_y,
                        );
                    }
                };
            }

            cvt!(proto::VideoBufferType::Rgba, i422_to_abgr);
            cvt!(proto::VideoBufferType::Abgr, i422_to_rgba);
            cvt!(proto::VideoBufferType::Argb, i422_to_bgra);

            Ok((dst, rgba_info(&dst, dst_type, width, height)))
        }
        proto::VideoBufferType::I420 => {
            let chroma_w = (width + 1) / 2;
            let chroma_h = (height + 1) / 2;
            let mut dst =
                vec![0u8; (width * height + chroma_w * chroma_h * 2) as usize].into_boxed_slice();
            let (dst_y, dst_u, dst_v) = split_i420_mut(&mut dst, width, chroma_w, chroma_w, height);

            colorcvt::i422_to_i420(
                y, c0.stride, u, c1.stride, v, c2.stride, dst_y, width, dst_u, chroma_w, dst_v,
                chroma_w, width, height, flip_y,
            );

            Ok((dst, i420_info(&dst, width, height)))
        }
        _ => {
            return Err(FfiError::InvalidRequest(
                format!("i422 to {:?} is not supported", dst_type).into(),
            ))
        }
    }
}

pub fn cvt_i444(
    buffer: proto::VideoBufferInfo,
    dst_type: proto::VideoBufferType,
    flip_y: bool,
) -> FfiResult<(Box<[u8]>, proto::VideoBufferInfo)> {
    assert_eq!(buffer.r#type(), proto::VideoBufferType::I444);
    let proto::VideoBufferInfo { width, height, components, data_ptr, data_len, .. } = buffer;
    let data = unsafe { slice::from_raw_parts(data_ptr as *const u8, data_len as usize) };

    let [c0, c1, c2, ..] = components.as_slice();
    let (y, u, v) = split_i444(data, c0.stride, c1.stride, c2.stride, height);

    match dst_type {
        proto::VideoBufferType::Rgba | proto::VideoBufferType::Bgra => {
            let mut dst = vec![0u8; (buffer.width * buffer.height * 4) as usize].into_boxed_slice();
            let stride = buffer.width * 4;

            macro_rules! cvt {
                ($rgba:expr, $fnc:ident) => {
                    if dst_type == $rgba {
                        imgproc::colorcvt::$fnc(
                            y, c0.stride, u, c1.stride, v, c2.stride, &mut dst, stride, width,
                            height, flip_y,
                        );
                    }
                };
            }

            cvt!(proto::VideoBufferType::Rgba, i444_to_abgr);
            cvt!(proto::VideoBufferType::Bgra, i444_to_argb);

            Ok((dst, rgba_info(&dst, dst_type, width, height)))
        }
        proto::VideoBufferType::I420 => {
            let chroma_w = (width + 1) / 2;
            let chroma_h = (height + 1) / 2;
            let mut dst =
                vec![0u8; (width * height + chroma_w * chroma_h * 2) as usize].into_boxed_slice();
            let (dst_y, dst_u, dst_v) = split_i420_mut(&mut dst, width, chroma_w, chroma_w, height);

            imgproc::colorcvt::i444_to_i420(
                y, c0.stride, u, c1.stride, v, c2.stride, dst_y, width, dst_u, chroma_w, dst_v,
                chroma_w, width, height, flip_y,
            );

            Ok((dst, i420_info(&dst, width, height)))
        }
        _ => {
            return Err(FfiError::InvalidRequest(
                format!("i444 to {:?} is not supported", dst_type).into(),
            ))
        }
    }
}

pub fn cvt_i010(
    buffer: proto::VideoBufferInfo,
    dst_type: proto::VideoBufferType,
    flip_y: bool,
) -> FfiResult<(Box<[u8]>, proto::VideoBufferInfo)> {
    assert_eq!(buffer.r#type(), proto::VideoBufferType::I010);
    let proto::VideoBufferInfo { width, height, components, data_ptr, data_len, .. } = buffer;
    let data = unsafe { slice::from_raw_parts(data_ptr as *const u8, data_len as usize) };

    let [c0, c1, c2, ..] = components.as_slice();
    let (y, u, v) = split_i010(data, c0.stride, c1.stride, c2.stride, height);

    let (_, y, _) = unsafe { y.align_to_mut::<u16>() };
    let (_, u, _) = unsafe { u.align_to_mut::<u16>() };
    let (_, v, _) = unsafe { v.align_to_mut::<u16>() };

    match dst_type {
        proto::VideoBufferType::Rgba | proto::VideoBufferType::Bgra => {
            let mut dst = vec![0u8; (buffer.width * buffer.height * 4) as usize].into_boxed_slice();
            let stride = buffer.width * 4;

            macro_rules! cvt {
                ($rgba:expr, $fnc:ident) => {
                    if dst_type == $rgba {
                        imgproc::colorcvt::$fnc(
                            y, c0.stride, u, c1.stride, v, c2.stride, &mut dst, stride, width,
                            height, flip_y,
                        );
                    }
                };
            }

            cvt!(proto::VideoBufferType::Rgba, i010_to_abgr);
            cvt!(proto::VideoBufferType::Bgra, i010_to_argb);

            Ok((dst, rgba_info(&dst, dst_type, width, height)))
        }
        proto::VideoBufferType::I420 => {
            let chroma_w = (buffer.width + 1) / 2;
            let chroma_h = (buffer.height + 1) / 2;
            let mut dst =
                vec![0u8; (width * height + chroma_w * chroma_h * 2) as usize].into_boxed_slice();
            let (dst_y, dst_u, dst_v) = split_i420_mut(&mut dst, width, chroma_w, chroma_w, height);

            imgproc::colorcvt::i010_to_i420(
                y, c0.stride, u, c1.stride, v, c2.stride, dst_y, width, dst_u, chroma_w, dst_v,
                chroma_w, width, height, flip_y,
            );

            Ok((dst, i420_info(&dst, width, height)))
        }
        _ => {
            return Err(FfiError::InvalidRequest(
                format!("i010 to {:?} is not supported", dst_type).into(),
            ))
        }
    }
}

pub fn cvt_nv12(
    buffer: proto::VideoBufferInfo,
    dst_type: proto::VideoBufferType,
    flip_y: bool,
) -> FfiResult<(Box<[u8]>, proto::VideoBufferInfo)> {
    assert_eq!(buffer.r#type(), proto::VideoBufferType::Nv12);
    let proto::VideoBufferInfo { width, height, components, data_ptr, data_len, .. } = buffer;
    let data = unsafe { slice::from_raw_parts(data_ptr as *const u8, data_len as usize) };

    let [c0, c1, ..] = components.as_slice();
    let (y, uv) = split_nv12(data, c0.stride, c1.stride, height);

    match dst_type {
        proto::VideoBufferType::Rgba | proto::VideoBufferType::Bgra => {
            let mut dst = vec![0u8; (buffer.width * buffer.height * 4) as usize].into_boxed_slice();
            let stride = buffer.width * 4;

            macro_rules! cvt {
                ($rgba:expr, $fnc:ident) => {
                    if dst_type == $rgba {
                        imgproc::colorcvt::$fnc(
                            y, c0.stride, uv, c1.stride, &mut dst, stride, width, height, flip_y,
                        );
                    }
                };
            }

            cvt!(proto::VideoBufferType::Rgba, nv12_to_abgr);
            cvt!(proto::VideoBufferType::Bgra, nv12_to_argb);

            Ok((dst, rgba_info(&dst, dst_type, width, height)))
        }
        proto::VideoBufferType::I420 => {
            let chroma_w = (buffer.width + 1) / 2;
            let chroma_h = (buffer.height + 1) / 2;
            let mut dst =
                vec![0u8; (width * height + chroma_w * chroma_h * 2) as usize].into_boxed_slice();
            let (dst_y, dst_u, dst_v) = split_i420_mut(&mut dst, width, chroma_w, chroma_w, height);

            imgproc::colorcvt::nv12_to_i420(
                y, c0.stride, uv, c1.stride, dst_y, width, dst_u, chroma_w, dst_v, chroma_w, width,
                height, flip_y,
            );

            Ok((dst, i420_info(&dst, width, height)))
        }
        _ => {
            return Err(FfiError::InvalidRequest(
                format!("nv12 to {:?} is not supported", dst_type).into(),
            ))
        }
    }
}
