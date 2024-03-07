/// Pixel format definitions.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, Default)]
pub enum PixelFormat {
    /// None
    #[default]
    None,
    /// Planar YUV 4:2:0, 12bpp, (1 Cr & Cb sample per 2x2 Y samples)
    YUV420P,
    /// Packed YUV 4:2:2, 16bpp, Y0 Cb Y1 Cr
    YUYV422,
    /// Packed RGB 8:8:8, 24bpp, RGBRGB...
    RGB24,
    /// Packed RGB 8:8:8, 24bpp, BGRBGR...
    BGR24,
    /// Planar YUV 4:2:2, 16bpp, (1 Cr & Cb sample per 2x1 Y samples)
    YUV422P,
    /// Planar YUV 4:4:4, 24bpp, (1 Cr & Cb sample per 1x1 Y samples)
    YUV444P,
    /// Planar YUV 4:1:0,  9bpp, (1 Cr & Cb sample per 4x4 Y samples)
    YUV410P,
    /// Planar YUV 4:1:1, 12bpp, (1 Cr & Cb sample per 4x1 Y samples)
    YUV411P,
    /// Y        ,  8bpp
    GRAY8,
    /// Y        ,  1bpp, 0 is white, 1 is black, in each byte pixels are ordered from the msb to the lsb
    MONOWHITE,
    /// Y        ,  1bpp, 0 is black, 1 is white, in each byte pixels are ordered from the msb to the lsb
    MONOBLACK,
    /// 8 bits with AV_PIX_FMT_RGB32 palette
    PAL8,
    /// Planar YUV 4:2:0, 12bpp, full scale (JPEG), deprecated in favor of AV_PIX_FMT_YUV420P and setting color_range
    YUVJ420P,
    /// Planar YUV 4:2:2, 16bpp, full scale (JPEG), deprecated in favor of AV_PIX_FMT_YUV422P and setting color_range
    YUVJ422P,
    /// Planar YUV 4:4:4, 24bpp, full scale (JPEG), deprecated in favor of AV_PIX_FMT_YUV444P and setting color_range
    YUVJ444P,
    /// Packed YUV 4:2:2, 16bpp, Cb Y0 Cr Y1
    UYVY422,
    /// Packed YUV 4:1:1, 12bpp, Cb Y0 Y1 Cr Y2 Y3
    UYYVYY411,
    /// Packed RGB 3:3:2,  8bpp, (msb)2B 3G 3R(lsb)
    BGR8,
    /// Packed RGB 1:2:1 bitstream,  4bpp, (msb)1B 2G 1R(lsb), a byte contains two pixels, the first pixel in the byte is the one composed by the 4 msb bits
    BGR4,
    /// Packed RGB 1:2:1,  8bpp, (msb)1B 2G 1R(lsb)
    BGR4_BYTE,
    /// Packed RGB 3:3:2,  8bpp, (msb)3R 3G 2B(lsb)
    RGB8,
    /// Packed RGB 1:2:1 bitstream,  4bpp, (msb)1R 2G 1B(lsb), a byte contains two pixels, the first pixel in the byte is the one composed by the 4 msb bits
    RGB4,
    /// Packed RGB 1:2:1,  8bpp, (msb)1R 2G 1B(lsb)
    RGB4_BYTE,
    /// Planar YUV 4:2:0, 12bpp, 1 plane for Y and 1 plane for the UV components, which are interleaved (first byte U and the following byte V)
    NV12,
    /// As above, but U and V bytes are swapped
    NV21,
    /// Packed ARGB 8:8:8:8, 32bpp, ARGBARGB...
    ARGB,
    /// Packed RGBA 8:8:8:8, 32bpp, RGBARGBA...
    RGBA,
    /// Packed ABGR 8:8:8:8, 32bpp, ABGRABGR...
    ABGR,
    /// Packed BGRA 8:8:8:8, 32bpp, BGRABGRA...
    BGRA,
    /// Y        , 16bpp, big-endian
    GRAY16BE,
    /// Y        , 16bpp, little-endian
    GRAY16LE,
    /// Planar YUV 4:4:0 (1 Cr & Cb sample per 1x2 Y samples)
    YUV440P,
    /// Planar YUV 4:4:0 full scale (JPEG), deprecated in favor of AV_PIX_FMT_YUV440P and setting color_range
    YUVJ440P,
    /// Planar YUV 4:2:0, 20bpp, (1 Cr & Cb sample per 2x2 Y & A samples)
    YUVA420P,
    /// Packed RGB 16:16:16, 48bpp, 16R, 16G, 16B, the 2-byte value for each R/G/B component is stored as big-endian
    RGB48BE,
    /// Packed RGB 16:16:16, 48bpp, 16R, 16G, 16B, the 2-byte value for each R/G/B component is stored as little-endian
    RGB48LE,
    /// Packed RGB 5:6:5, 16bpp, (msb)   5R 6G 5B(lsb), big-endian
    RGB565BE,
    /// Packed RGB 5:6:5, 16bpp, (msb)   5R 6G 5B(lsb), little-endian
    RGB565LE,
    /// Packed RGB 5:5:5, 16bpp, (msb)1X 5R 5G 5B(lsb), big-endian   , X=unused/undefined
    RGB555BE,
    /// Packed RGB 5:5:5, 16bpp, (msb)1X 5R 5G 5B(lsb), little-endian, X=unused/undefined
    RGB555LE,
    /// Packed BGR 5:6:5, 16bpp, (msb)   5B 6G 5R(lsb), big-endian
    BGR565BE,
    /// Packed BGR 5:6:5, 16bpp, (msb)   5B 6G 5R(lsb), little-endian
    BGR565LE,
    /// Packed BGR 5:5:5, 16bpp, (msb)1X 5B 5G 5R(lsb), big-endian   , X=unused/undefined
    BGR555BE,
    /// Packed BGR 5:5:5, 16bpp, (msb)1X 5B 5G 5R(lsb), little-endian, X=unused/undefined
    BGR555LE,
    /// VAAPI
    VAAPI,
    /// Planar YUV 4:2:0, 24bpp, (1 Cr & Cb sample per 2x2 Y samples), little-endian
    YUV420P16LE,
    /// Planar YUV 4:2:0, 24bpp, (1 Cr & Cb sample per 2x2 Y samples), big-endian
    YUV420P16BE,
    /// Planar YUV 4:2:2, 32bpp, (1 Cr & Cb sample per 2x1 Y samples), little-endian
    YUV422P16LE,
    /// Planar YUV 4:2:2, 32bpp, (1 Cr & Cb sample per 2x1 Y samples), big-endian
    YUV422P16BE,
    /// Planar YUV 4:4:4, 48bpp, (1 Cr & Cb sample per 1x1 Y samples), little-endian
    YUV444P16LE,
    /// Planar YUV 4:4:4, 48bpp, (1 Cr & Cb sample per 1x1 Y samples), big-endian
    YUV444P16BE,
    /// HW decoding through DXVA2, Picture.data[3] contains a LPDIRECT3DSURFACE9 pointer
    DXVA2_VLD,
    /// Packed RGB 4:4:4, 16bpp, (msb)4X 4R 4G 4B(lsb), little-endian, X=unused/undefined
    RGB444LE,
    /// Packed RGB 4:4:4, 16bpp, (msb)4X 4R 4G 4B(lsb), big-endian,    X=unused/undefined
    RGB444BE,
    /// Packed BGR 4:4:4, 16bpp, (msb)4X 4B 4G 4R(lsb), little-endian, X=unused/undefined
    BGR444LE,
    /// Packed BGR 4:4:4, 16bpp, (msb)4X 4B 4G 4R(lsb), big-endian,    X=unused/undefined
    BGR444BE,
    /// 8 bits gray, 8 bits alpha
    YA8,
    /// Alias for AV_PIX_FMT_YA8
    Y400A,
    /// Alias for AV_PIX_FMT_YA8
    GRAY8A,
    /// Packed RGB 16:16:16, 48bpp, 16B, 16G, 16R, the 2-byte value for each R/G/B component is stored as big-endian
    BGR48BE,
    /// Packed RGB 16:16:16, 48bpp, 16B, 16G, 16R, the 2-byte value for each R/G/B component is stored as little-endian
    BGR48LE,
    /// Planar YUV 4:2:0, 13.5bpp, (1 Cr & Cb sample per 2x2 Y samples), big-endian
    YUV420P9BE,
    /// Planar YUV 4:2:0, 13.5bpp, (1 Cr & Cb sample per 2x2 Y samples), little-endian
    YUV420P9LE,
    /// Planar YUV 4:2:0, 15bpp, (1 Cr & Cb sample per 2x2 Y samples), big-endian
    YUV420P10BE,
    /// Planar YUV 4:2:0, 15bpp, (1 Cr & Cb sample per 2x2 Y samples), little-endian
    YUV420P10LE,
    /// Planar YUV 4:2:2, 20bpp, (1 Cr & Cb sample per 2x1 Y samples), big-endian
    YUV422P10BE,
    /// Planar YUV 4:2:2, 20bpp, (1 Cr & Cb sample per 2x1 Y samples), little-endian
    YUV422P10LE,
    /// Planar YUV 4:4:4, 27bpp, (1 Cr & Cb sample per 1x1 Y samples), big-endian
    YUV444P9BE,
    /// Planar YUV 4:4:4, 27bpp, (1 Cr & Cb sample per 1x1 Y samples), little-endian
    YUV444P9LE,
    /// Planar YUV 4:4:4, 30bpp, (1 Cr & Cb sample per 1x1 Y samples), big-endian
    YUV444P10BE,
    /// Planar YUV 4:4:4, 30bpp, (1 Cr & Cb sample per 1x1 Y samples), little-endian
    YUV444P10LE,
    /// Planar YUV 4:2:2, 18bpp, (1 Cr & Cb sample per 2x1 Y samples), big-endian
    YUV422P9BE,
    /// Planar YUV 4:2:2, 18bpp, (1 Cr & Cb sample per 2x1 Y samples), little-endian
    YUV422P9LE,
    /// Planar GBR 4:4:4 24bpp
    GBRP,
    /// Alias for #AV_PIX_FMT_GBRP
    GBR24P,
    /// Planar GBR 4:4:4 27bpp, big-endian
    GBRP9BE,
    /// Planar GBR 4:4:4 27bpp, little-endian
    GBRP9LE,
    /// Planar GBR 4:4:4 30bpp, big-endian
    GBRP10BE,
    /// Planar GBR 4:4:4 30bpp, little-endian
    GBRP10LE,
    /// Planar GBR 4:4:4 48bpp, big-endian
    GBRP16BE,
    /// Planar GBR 4:4:4 48bpp, little-endian
    GBRP16LE,
    /// Planar YUV 4:2:2 24bpp, (1 Cr & Cb sample per 2x1 Y & A samples)
    YUVA422P,
    /// Planar YUV 4:4:4 32bpp, (1 Cr & Cb sample per 1x1 Y & A samples)
    YUVA444P,
    /// Planar YUV 4:2:0 22.5bpp, (1 Cr & Cb sample per 2x2 Y & A samples), big-endian
    YUVA420P9BE,
    /// Planar YUV 4:2:0 22.5bpp, (1 Cr & Cb sample per 2x2 Y & A samples), little-endian
    YUVA420P9LE,
    /// Planar YUV 4:2:2 27bpp, (1 Cr & Cb sample per 2x1 Y & A samples), big-endian
    YUVA422P9BE,
    /// Planar YUV 4:2:2 27bpp, (1 Cr & Cb sample per 2x1 Y & A samples), little-endian
    YUVA422P9LE,
    /// Planar YUV 4:4:4 36bpp, (1 Cr & Cb sample per 1x1 Y & A samples), big-endian
    YUVA444P9BE,
    /// Planar YUV 4:4:4 36bpp, (1 Cr & Cb sample per 1x1 Y & A samples), little-endian
    YUVA444P9LE,
    /// Planar YUV 4:2:0 25bpp, (1 Cr & Cb sample per 2x2 Y & A samples, big-endian)
    YUVA420P10BE,
    /// Planar YUV 4:2:0 25bpp, (1 Cr & Cb sample per 2x2 Y & A samples, little-endian)
    YUVA420P10LE,
    /// Planar YUV 4:2:2 30bpp, (1 Cr & Cb sample per 2x1 Y & A samples, big-endian)
    YUVA422P10BE,
    /// Planar YUV 4:2:2 30bpp, (1 Cr & Cb sample per 2x1 Y & A samples, little-endian)
    YUVA422P10LE,
    /// Planar YUV 4:4:4 40bpp, (1 Cr & Cb sample per 1x1 Y & A samples, big-endian)
    YUVA444P10BE,
    /// Planar YUV 4:4:4 40bpp, (1 Cr & Cb sample per 1x1 Y & A samples, little-endian)
    YUVA444P10LE,
    /// Planar YUV 4:2:0 40bpp, (1 Cr & Cb sample per 2x2 Y & A samples, big-endian)
    YUVA420P16BE,
    /// Planar YUV 4:2:0 40bpp, (1 Cr & Cb sample per 2x2 Y & A samples, little-endian)
    YUVA420P16LE,
    /// Planar YUV 4:2:2 48bpp, (1 Cr & Cb sample per 2x1 Y & A samples, big-endian)
    YUVA422P16BE,
    /// Planar YUV 4:2:2 48bpp, (1 Cr & Cb sample per 2x1 Y & A samples, little-endian)
    YUVA422P16LE,
    /// Planar YUV 4:4:4 64bpp, (1 Cr & Cb sample per 1x1 Y & A samples, big-endian)
    YUVA444P16BE,
    /// Planar YUV 4:4:4 64bpp, (1 Cr & Cb sample per 1x1 Y & A samples, little-endian)
    YUVA444P16LE,
    /// HW acceleration through VDPAU, Picture.data[3] contains a VdpVideoSurface
    VDPAU,
    /// Packed XYZ 4:4:4, 36 bpp, (msb) 12X, 12Y, 12Z (lsb), the 2-byte value for each X/Y/Z is stored as little-endian, the 4 lower bits are set to 0
    XYZ12LE,
    /// Packed XYZ 4:4:4, 36 bpp, (msb) 12X, 12Y, 12Z (lsb), the 2-byte value for each X/Y/Z is stored as big-endian, the 4 lower bits are set to 0
    XYZ12BE,
    /// Interleaved chroma YUV 4:2:2, 16bpp, (1 Cr & Cb sample per 2x1 Y samples)
    NV16,
    /// Interleaved chroma YUV 4:2:2, 20bpp, (1 Cr & Cb sample per 2x1 Y samples), little-endian
    NV20LE,
    /// Interleaved chroma YUV 4:2:2, 20bpp, (1 Cr & Cb sample per 2x1 Y samples), big-endian
    NV20BE,
    /// Packed RGBA 16:16:16:16, 64bpp, 16R, 16G, 16B, 16A, the 2-byte value for each R/G/B/A component is stored as big-endian
    RGBA64BE,
    /// Packed RGBA 16:16:16:16, 64bpp, 16R, 16G, 16B, 16A, the 2-byte value for each R/G/B/A component is stored as little-endian
    RGBA64LE,
    /// Packed RGBA 16:16:16:16, 64bpp, 16B, 16G, 16R, 16A, the 2-byte value for each R/G/B/A component is stored as big-endian
    BGRA64BE,
    /// Packed RGBA 16:16:16:16, 64bpp, 16B, 16G, 16R, 16A, the 2-byte value for each R/G/B/A component is stored as little-endian
    BGRA64LE,
    /// Packed YUV 4:2:2, 16bpp, Y0 Cr Y1 Cb
    YVYU422,
    /// 16 bits gray, 16 bits alpha (big-endian)
    YA16BE,
    /// 16 bits gray, 16 bits alpha (little-endian)
    YA16LE,
    /// Planar GBRA 4:4:4:4 32bpp
    GBRAP,
    /// Planar GBRA 4:4:4:4 64bpp, big-endian
    GBRAP16BE,
    /// Planar GBRA 4:4:4:4 64bpp, little-endian
    GBRAP16LE,
    /// QSV
    QSV,
    /// MMAL
    MMAL,
    /// HW decoding through Direct3D11 via old API, Picture.data[3] contains a ID3D11VideoDecoderOutputView pointer
    D3D11VA_VLD,
    /// CUDA
    CUDA,
    /// Packed RGB 8:8:8, 32bpp, XRGBXRGB...   X=unused/undefined
    ZERORGB,
    /// Packed RGB 8:8:8, 32bpp, RGBXRGBX...   X=unused/undefined
    RGB0,
    /// Packed BGR 8:8:8, 32bpp, XBGRXBGR...   X=unused/undefined
    ZEROBGR,
    /// Packed BGR 8:8:8, 32bpp, BGRXBGRX...   X=unused/undefined
    BGR0,
    /// Planar YUV 4:2:0,18bpp, (1 Cr & Cb sample per 2x2 Y samples), big-endian
    YUV420P12BE,
    /// Planar YUV 4:2:0,18bpp, (1 Cr & Cb sample per 2x2 Y samples), little-endian
    YUV420P12LE,
    /// Planar YUV 4:2:0,21bpp, (1 Cr & Cb sample per 2x2 Y samples), big-endian
    YUV420P14BE,
    /// Planar YUV 4:2:0,21bpp, (1 Cr & Cb sample per 2x2 Y samples), little-endian
    YUV420P14LE,
    /// Planar YUV 4:2:2,24bpp, (1 Cr & Cb sample per 2x1 Y samples), big-endian
    YUV422P12BE,
    /// Planar YUV 4:2:2,24bpp, (1 Cr & Cb sample per 2x1 Y samples), little-endian
    YUV422P12LE,
    /// Planar YUV 4:2:2,28bpp, (1 Cr & Cb sample per 2x1 Y samples), big-endian
    YUV422P14BE,
    /// Planar YUV 4:2:2,28bpp, (1 Cr & Cb sample per 2x1 Y samples), little-endian
    YUV422P14LE,
    /// Planar YUV 4:4:4,36bpp, (1 Cr & Cb sample per 1x1 Y samples), big-endian
    YUV444P12BE,
    /// Planar YUV 4:4:4,36bpp, (1 Cr & Cb sample per 1x1 Y samples), little-endian
    YUV444P12LE,
    /// Planar YUV 4:4:4,42bpp, (1 Cr & Cb sample per 1x1 Y samples), big-endian
    YUV444P14BE,
    /// Planar YUV 4:4:4,42bpp, (1 Cr & Cb sample per 1x1 Y samples), little-endian
    YUV444P14LE,
    /// Planar GBR 4:4:4 36bpp, big-endian
    GBRP12BE,
    /// Planar GBR 4:4:4 36bpp, little-endian
    GBRP12LE,
    /// Planar GBR 4:4:4 42bpp, big-endian
    GBRP14BE,
    /// Planar GBR 4:4:4 42bpp, little-endian
    GBRP14LE,
    /// Planar YUV 4:1:1, 12bpp, (1 Cr & Cb sample per 4x1 Y samples) full scale (JPEG), deprecated in favor of AV_PIX_FMT_YUV411P and setting color_range
    YUVJ411P,
    /// Bayer, BGBG..(odd line), GRGR..(even line), 8-bit samples
    BAYER_BGGR8,
    /// Bayer, RGRG..(odd line), GBGB..(even line), 8-bit samples
    BAYER_RGGB8,
    /// Bayer, GBGB..(odd line), RGRG..(even line), 8-bit samples
    BAYER_GBRG8,
    /// Bayer, GRGR..(odd line), BGBG..(even line), 8-bit samples
    BAYER_GRBG8,
    /// Bayer, BGBG..(odd line), GRGR..(even line), 16-bit samples, little-endian
    BAYER_BGGR16LE,
    /// Bayer, BGBG..(odd line), GRGR..(even line), 16-bit samples, big-endian
    BAYER_BGGR16BE,
    /// Bayer, RGRG..(odd line), GBGB..(even line), 16-bit samples, little-endian
    BAYER_RGGB16LE,
    /// Bayer, RGRG..(odd line), GBGB..(even line), 16-bit samples, big-endian
    BAYER_RGGB16BE,
    /// Bayer, GBGB..(odd line), RGRG..(even line), 16-bit samples, little-endian
    BAYER_GBRG16LE,
    /// Bayer, GBGB..(odd line), RGRG..(even line), 16-bit samples, big-endian
    BAYER_GBRG16BE,
    /// Bayer, GRGR..(odd line), BGBG..(even line), 16-bit samples, little-endian
    BAYER_GRBG16LE,
    /// Bayer, GRGR..(odd line), BGBG..(even line), 16-bit samples, big-endian
    BAYER_GRBG16BE,
    /// Planar YUV 4:4:0,20bpp, (1 Cr & Cb sample per 1x2 Y samples), little-endian
    YUV440P10LE,
    /// Planar YUV 4:4:0,20bpp, (1 Cr & Cb sample per 1x2 Y samples), big-endian
    YUV440P10BE,
    /// Planar YUV 4:4:0,24bpp, (1 Cr & Cb sample per 1x2 Y samples), little-endian
    YUV440P12LE,
    /// Planar YUV 4:4:0,24bpp, (1 Cr & Cb sample per 1x2 Y samples), big-endian
    YUV440P12BE,
    /// Packed AYUV 4:4:4,64bpp (1 Cr & Cb sample per 1x1 Y & A samples), little-endian
    AYUV64LE,
    /// Packed AYUV 4:4:4,64bpp (1 Cr & Cb sample per 1x1 Y & A samples), big-endian
    AYUV64BE,
    /// Hardware decoding through Videotoolbox
    VIDEOTOOLBOX,
    /// Like NV12, with 10bpp per component, data in the high bits, zeros in the low bits, little-endian
    P010LE,
    /// Like NV12, with 10bpp per component, data in the high bits, zeros in the low bits, big-endian
    P010BE,
    /// Planar GBR 4:4:4:4 48bpp, big-endian
    GBRAP12BE,
    /// Planar GBR 4:4:4:4 48bpp, little-endian
    GBRAP12LE,
    /// Planar GBR 4:4:4:4 40bpp, big-endian
    GBRAP10BE,
    /// Planar GBR 4:4:4:4 40bpp, little-endian
    GBRAP10LE,
    /// Hardware decoding through MediaCodec
    MEDIACODEC,
    /// Y        , 12bpp, big-endian
    GRAY12BE,
    /// Y        , 12bpp, little-endian
    GRAY12LE,
    /// Y        , 10bpp, big-endian
    GRAY10BE,
    /// Y        , 10bpp, little-endian
    GRAY10LE,
    /// Like NV12, with 16bpp per component, little-endian
    P016LE,
    /// Like NV12, with 16bpp per component, big-endian
    P016BE,
    /// D3D11
    D3D11,
    /// Y        , 9bpp, big-endian
    GRAY9BE,
    /// Y        , 9bpp, little-endian
    GRAY9LE,
    /// IEEE-754 single precision planar GBR 4:4:4,     96bpp, big-endian
    GBRPF32BE,
    /// IEEE-754 single precision planar GBR 4:4:4,     96bpp, little-endian
    GBRPF32LE,
    /// IEEE-754 single precision planar GBRA 4:4:4:4, 128bpp, big-endian
    GBRAPF32BE,
    /// IEEE-754 single precision planar GBRA 4:4:4:4, 128bpp, little-endian
    GBRAPF32LE,
    /// DRM_PRIME
    DRM_PRIME,
    /// OPENCL
    OPENCL,
    /// Y        , 14bpp, big-endian
    GRAY14BE,
    /// Y        , 14bpp, little-endian
    GRAY14LE,
    /// IEEE-754 single precision Y, 32bpp, big-endian
    GRAYF32BE,
    /// IEEE-754 single precision Y, 32bpp, little-endian
    GRAYF32LE,
    /// Planar YUV 4:2:2,24bpp, (1 Cr & Cb sample per 2x1 Y samples), 12b alpha, big-endian
    YUVA422P12BE,
    /// Planar YUV 4:2:2,24bpp, (1 Cr & Cb sample per 2x1 Y samples), 12b alpha, little-endian
    YUVA422P12LE,
    /// Planar YUV 4:4:4,36bpp, (1 Cr & Cb sample per 1x1 Y samples), 12b alpha, big-endian
    YUVA444P12BE,
    /// Planar YUV 4:4:4,36bpp, (1 Cr & Cb sample per 1x1 Y samples), 12b alpha, little-endian
    YUVA444P12LE,
    /// Planar YUV 4:4:4, 24bpp, 1 plane for Y and 1 plane for the UV components, which are interleaved (first byte U and the following byte V)
    NV24,
    /// As above, but U and V bytes are swapped
    NV42,
    /// VULKAN
    VULKAN,
    /// Packed YUV 4:2:2 like YUYV422, 20bpp, data in the high bits, big-endian
    Y210BE,
    /// Packed YUV 4:2:2 like YUYV422, 20bpp, data in the high bits, little-endian
    Y210LE,
    /// Packed RGB 10:10:10, 30bpp, (msb)2X 10R 10G 10B(lsb), little-endian, X=unused/undefined
    X2RGB10LE,
    /// Packed RGB 10:10:10, 30bpp, (msb)2X 10R 10G 10B(lsb), big-endian, X=unused/undefined
    X2RGB10BE,
    /// Packed BGR 10:10:10, 30bpp, (msb)2X 10B 10G 10R(lsb), little-endian, X=unused/undefined
    X2BGR10LE,
    /// Packed BGR 10:10:10, 30bpp, (msb)2X 10B 10G 10R(lsb), big-endian, X=unused/undefined
    X2BGR10BE,
    /// Interleaved chroma YUV 4:2:2, 20bpp, data in the high bits, big-endian
    P210BE,
    /// Interleaved chroma YUV 4:2:2, 20bpp, data in the high bits, little-endian
    P210LE,
    /// Interleaved chroma YUV 4:4:4, 30bpp, data in the high bits, big-endian
    P410BE,
    /// Interleaved chroma YUV 4:4:4, 30bpp, data in the high bits, little-endian
    P410LE,
    /// Interleaved chroma YUV 4:2:2, 32bpp, big-endian
    P216BE,
    /// Interleaved chroma YUV 4:2:2, 32bpp, little-endian
    P216LE,
    /// Interleaved chroma YUV 4:4:4, 48bpp, big-endian
    P416BE,
    /// Interleaved chroma YUV 4:4:4, 48bpp, little-endian
    P416LE,
    /// Packed VUYA 4:4:4, 32bpp, VUYAVUYA...
    VUYA,
    /// IEEE-754 half precision packed RGBA 16:16:16:16, 64bpp, RGBARGBA..., big-endian
    RGBAF16BE,
    /// IEEE-754 half precision packed RGBA 16:16:16:16, 64bpp, RGBARGBA..., little-endian
    RGBAF16LE,
    /// Packed VUYX 4:4:4, 32bpp, Variant of VUYA where alpha channel is left undefined
    VUYX,
    /// Like NV12, with 12bpp per component, data in the high bits, zeros in the low bits, little-endian
    P012LE,
    /// Like NV12, with 12bpp per component, data in the high bits, zeros in the low bits, big-endian
    P012BE,
    /// Packed YUV 4:2:2 like YUYV422, 24bpp, data in the high bits, zeros in the low bits, big-endian
    Y212BE,
    /// Packed YUV 4:2:2 like YUYV422, 24bpp, data in the high bits, zeros in the low bits, little-endian
    Y212LE,
    /// Packed XVYU 4:4:4, 32bpp, (msb)2X 10V 10Y 10U(lsb), big-endian, variant of Y410 where alpha channel is left undefined
    XV30BE,
    /// Packed XVYU 4:4:4, 32bpp, (msb)2X 10V 10Y 10U(lsb), little-endian, variant of Y410 where alpha channel is left undefined
    XV30LE,
    /// Packed XVYU 4:4:4, 48bpp, data in the high bits, zeros in the low bits, big-endian, variant of Y412 where alpha channel is left undefined
    XV36BE,
    /// Packed XVYU 4:4:4, 48bpp, data in the high bits, zeros in the low bits, little-endian, variant of Y412 where alpha channel is left undefined
    XV36LE,
    /// IEEE-754 single precision packed RGB 32:32:32, 96bpp, RGBRGB..., big-endian
    RGBF32BE,
    /// IEEE-754 single precision packed RGB 32:32:32, 96bpp, RGBRGB..., little-endian
    RGBF32LE,
    /// IEEE-754 single precision packed RGBA 32:32:32:32, 128bpp, RGBARGBA..., big-endian
    RGBAF32BE,
    /// IEEE-754 single precision packed RGBA 32:32:32:32, 128bpp, RGBARGBA..., little-endian
    RGBAF32LE,
    /// Interleaved chroma YUV 4:2:2, 24bpp, data in the high bits, big-endian
    P212BE,
    /// Interleaved chroma YUV 4:2:2, 24bpp, data in the high bits, little-endian
    P212LE,
    /// Interleaved chroma YUV 4:4:4, 36bpp, data in the high bits, big-endian
    P412BE,
    /// Interleaved chroma YUV 4:4:4, 36bpp, data in the high bits, little-endian
    P412LE,
    /// Planar GBR 4:4:4:4 56bpp, big-endian
    GBRAP14BE,
    /// Planar GBR 4:4:4:4 56bpp, little-endian
    GBRAP14LE,
    /// D3D12
    D3D12,
    /// Number of pixel formats, DO NOT USE THIS if you want to link with shared libav* because the number of formats might differ between versions
    NB,
}

impl PixelFormat {
    pub(crate) const fn new(pixel_format: i32) -> Self {
        match pixel_format {
            -1 => Self::None,
            0 => Self::YUV420P,
            1 => Self::YUYV422,
            2 => Self::RGB24,
            3 => Self::BGR24,
            4 => Self::YUV422P,
            5 => Self::YUV444P,
            6 => Self::YUV410P,
            7 => Self::YUV411P,
            8 => Self::GRAY8,
            9 => Self::MONOWHITE,
            10 => Self::MONOBLACK,
            11 => Self::PAL8,
            12 => Self::YUVJ420P,
            13 => Self::YUVJ422P,
            14 => Self::YUVJ444P,
            15 => Self::UYVY422,
            16 => Self::UYYVYY411,
            17 => Self::BGR8,
            18 => Self::BGR4,
            19 => Self::BGR4_BYTE,
            20 => Self::RGB8,
            21 => Self::RGB4,
            22 => Self::RGB4_BYTE,
            23 => Self::NV12,
            24 => Self::NV21,
            25 => Self::ARGB,
            26 => Self::RGBA,
            27 => Self::ABGR,
            28 => Self::BGRA,
            29 => Self::GRAY16BE,
            30 => Self::GRAY16LE,
            31 => Self::YUV440P,
            32 => Self::YUVJ440P,
            33 => Self::YUVA420P,
            34 => Self::RGB48BE,
            35 => Self::RGB48LE,
            36 => Self::RGB565BE,
            37 => Self::RGB565LE,
            38 => Self::RGB555BE,
            39 => Self::RGB555LE,
            40 => Self::BGR565BE,
            41 => Self::BGR565LE,
            42 => Self::BGR555BE,
            43 => Self::BGR555LE,
            44 => Self::VAAPI,
            45 => Self::YUV420P16LE,
            46 => Self::YUV420P16BE,
            47 => Self::YUV422P16LE,
            48 => Self::YUV422P16BE,
            49 => Self::YUV444P16LE,
            50 => Self::YUV444P16BE,
            51 => Self::DXVA2_VLD,
            52 => Self::RGB444LE,
            53 => Self::RGB444BE,
            54 => Self::BGR444LE,
            55 => Self::BGR444BE,
            56 => Self::YA8,
            57 => Self::Y400A,
            58 => Self::GRAY8A,
            59 => Self::BGR48BE,
            60 => Self::BGR48LE,
            61 => Self::YUV420P9BE,
            62 => Self::YUV420P9LE,
            63 => Self::YUV420P10BE,
            64 => Self::YUV420P10LE,
            65 => Self::YUV422P10BE,
            66 => Self::YUV422P10LE,
            67 => Self::YUV444P9BE,
            68 => Self::YUV444P9LE,
            69 => Self::YUV444P10BE,
            70 => Self::YUV444P10LE,
            71 => Self::YUV422P9BE,
            72 => Self::YUV422P9LE,
            73 => Self::GBRP,
            74 => Self::GBR24P,
            75 => Self::GBRP9BE,
            76 => Self::GBRP9LE,
            77 => Self::GBRP10BE,
            78 => Self::GBRP10LE,
            79 => Self::GBRP16BE,
            80 => Self::GBRP16LE,
            81 => Self::YUVA422P,
            82 => Self::YUVA444P,
            83 => Self::YUVA420P9BE,
            84 => Self::YUVA420P9LE,
            85 => Self::YUVA422P9BE,
            86 => Self::YUVA422P9LE,
            87 => Self::YUVA444P9BE,
            88 => Self::YUVA444P9LE,
            89 => Self::YUVA420P10BE,
            90 => Self::YUVA420P10LE,
            91 => Self::YUVA422P10BE,
            92 => Self::YUVA422P10LE,
            93 => Self::YUVA444P10BE,
            94 => Self::YUVA444P10LE,
            95 => Self::YUVA420P16BE,
            96 => Self::YUVA420P16LE,
            97 => Self::YUVA422P16BE,
            98 => Self::YUVA422P16LE,
            99 => Self::YUVA444P16BE,
            100 => Self::YUVA444P16LE,
            101 => Self::VDPAU,
            102 => Self::XYZ12LE,
            103 => Self::XYZ12BE,
            104 => Self::NV16,
            105 => Self::NV20LE,
            106 => Self::NV20BE,
            107 => Self::RGBA64BE,
            108 => Self::RGBA64LE,
            109 => Self::BGRA64BE,
            110 => Self::BGRA64LE,
            111 => Self::YVYU422,
            112 => Self::YA16BE,
            113 => Self::YA16LE,
            114 => Self::GBRAP,
            115 => Self::GBRAP16BE,
            116 => Self::GBRAP16LE,
            117 => Self::QSV,
            118 => Self::MMAL,
            119 => Self::D3D11VA_VLD,
            120 => Self::CUDA,
            121 => Self::ZERORGB,
            122 => Self::RGB0,
            123 => Self::ZEROBGR,
            124 => Self::BGR0,
            125 => Self::YUV420P12BE,
            126 => Self::YUV420P12LE,
            127 => Self::YUV420P14BE,
            128 => Self::YUV420P14LE,
            129 => Self::YUV422P12BE,
            130 => Self::YUV422P12LE,
            131 => Self::YUV422P14BE,
            132 => Self::YUV422P14LE,
            133 => Self::YUV444P12BE,
            134 => Self::YUV444P12LE,
            135 => Self::YUV444P14BE,
            136 => Self::YUV444P14LE,
            137 => Self::GBRP12BE,
            138 => Self::GBRP12LE,
            139 => Self::GBRP14BE,
            140 => Self::GBRP14LE,
            141 => Self::YUVJ411P,
            142 => Self::BAYER_BGGR8,
            143 => Self::BAYER_RGGB8,
            144 => Self::BAYER_GBRG8,
            145 => Self::BAYER_GRBG8,
            146 => Self::BAYER_BGGR16LE,
            147 => Self::BAYER_BGGR16BE,
            148 => Self::BAYER_RGGB16LE,
            149 => Self::BAYER_RGGB16BE,
            150 => Self::BAYER_GBRG16LE,
            151 => Self::BAYER_GBRG16BE,
            152 => Self::BAYER_GRBG16LE,
            153 => Self::BAYER_GRBG16BE,
            154 => Self::YUV440P10LE,
            155 => Self::YUV440P10BE,
            156 => Self::YUV440P12LE,
            157 => Self::YUV440P12BE,
            158 => Self::AYUV64LE,
            159 => Self::AYUV64BE,
            160 => Self::VIDEOTOOLBOX,
            161 => Self::P010LE,
            162 => Self::P010BE,
            163 => Self::GBRAP12BE,
            164 => Self::GBRAP12LE,
            165 => Self::GBRAP10BE,
            166 => Self::GBRAP10LE,
            167 => Self::MEDIACODEC,
            168 => Self::GRAY12BE,
            169 => Self::GRAY12LE,
            170 => Self::GRAY10BE,
            171 => Self::GRAY10LE,
            172 => Self::P016LE,
            173 => Self::P016BE,
            174 => Self::D3D11,
            175 => Self::GRAY9BE,
            176 => Self::GRAY9LE,
            177 => Self::GBRPF32BE,
            178 => Self::GBRPF32LE,
            179 => Self::GBRAPF32BE,
            180 => Self::GBRAPF32LE,
            181 => Self::DRM_PRIME,
            182 => Self::OPENCL,
            183 => Self::GRAY14BE,
            184 => Self::GRAY14LE,
            185 => Self::GRAYF32BE,
            186 => Self::GRAYF32LE,
            187 => Self::YUVA422P12BE,
            188 => Self::YUVA422P12LE,
            189 => Self::YUVA444P12BE,
            190 => Self::YUVA444P12LE,
            191 => Self::NV24,
            192 => Self::NV42,
            193 => Self::VULKAN,
            194 => Self::Y210BE,
            195 => Self::Y210LE,
            196 => Self::X2RGB10LE,
            197 => Self::X2RGB10BE,
            198 => Self::X2BGR10LE,
            199 => Self::X2BGR10BE,
            200 => Self::P210BE,
            201 => Self::P210LE,
            202 => Self::P410BE,
            203 => Self::P410LE,
            204 => Self::P216BE,
            205 => Self::P216LE,
            206 => Self::P416BE,
            207 => Self::P416LE,
            208 => Self::VUYA,
            209 => Self::RGBAF16BE,
            210 => Self::RGBAF16LE,
            211 => Self::VUYX,
            212 => Self::P012LE,
            213 => Self::P012BE,
            214 => Self::Y212BE,
            215 => Self::Y212LE,
            216 => Self::XV30BE,
            217 => Self::XV30LE,
            218 => Self::XV36BE,
            219 => Self::XV36LE,
            220 => Self::RGBF32BE,
            221 => Self::RGBF32LE,
            222 => Self::RGBAF32BE,
            223 => Self::RGBAF32LE,
            224 => Self::P212BE,
            225 => Self::P212LE,
            226 => Self::P412BE,
            227 => Self::P412LE,
            228 => Self::GBRAP14BE,
            229 => Self::GBRAP14LE,
            230 => Self::D3D12,
            231 => Self::NB,
            _ => Self::None,
        }
    }
}
