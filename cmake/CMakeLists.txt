cmake_minimum_required(VERSION 2.8)

project(h264_decoder)

include_directories(
    "../h264_decoder/src/src/"
	"../h264_decoder/src/inc/"
)

file(
    GLOB
    sources
	"../h264_decoder/src/src/h264bsd_transform.c"
	"../h264_decoder/src/src/h264bsd_util.c"
	"../h264_decoder/src/src/h264bsd_byte_stream.c"
	"../h264_decoder/src/src/h264bsd_seq_param_set.c"
	"../h264_decoder/src/src/h264bsd_pic_param_set.c"
	"../h264_decoder/src/src/h264bsd_slice_header.c"
	"../h264_decoder/src/src/h264bsd_slice_data.c"
	"../h264_decoder/src/src/h264bsd_macroblock_layer.c"
	"../h264_decoder/src/src/h264bsd_stream.c"
	"../h264_decoder/src/src/h264bsd_vlc.c"
	"../h264_decoder/src/src/h264bsd_cavlc.c"
	"../h264_decoder/src/src/h264bsd_nal_unit.c"
	"../h264_decoder/src/src/h264bsd_neighbour.c"
	"../h264_decoder/src/src/h264bsd_storage.c"
	"../h264_decoder/src/src/h264bsd_slice_group_map.c"
	"../h264_decoder/src/src/h264bsd_intra_prediction.c"
	"../h264_decoder/src/src/h264bsd_inter_prediction.c"
	"../h264_decoder/src/src/h264bsd_reconstruct.c"
	"../h264_decoder/src/src/h264bsd_dpb.c"
	"../h264_decoder/src/src/h264bsd_image.c"
	"../h264_decoder/src/src/h264bsd_deblocking.c"
	"../h264_decoder/src/src/h264bsd_conceal.c"
	"../h264_decoder/src/src/h264bsd_vui.c"
	"../h264_decoder/src/src/h264bsd_pic_order_cnt.c"
	"../h264_decoder/src/src/h264bsd_decoder.c"
	"../h264_decoder/src/src/H264SwDecApi.c"
	"../h264_decoder/src/src/extraFlags.c"
)

add_library(
    h264_decoder
    STATIC
    ${sources}
)
