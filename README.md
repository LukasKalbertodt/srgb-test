# sRGB `glutin` test

Minimal example to test a potential bug of `glutin`.
See [this issue](https://github.com/rust-windowing/glutin/issues/1175) for more information.

When executing this with `cargo run`, there are two data points of interest:

- The output on stdout
- The color of the output. Take a screenshot or somehow inspect the color.
  It *should* be `#808080`. But it's probably `#bbbbbb` or something like that.


## Results

<table>
  <thead>
    <tr>
      <th></th>
      <th><code>with_srgb(false)</code></th>
      <th><code>with_srgb(true)</code></th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>Ubuntu 18.04</td>
      <td><code>#bbbbbb</code><br><pre>What OpenGL thinks: srgb (35904)
What glutin thinks: srgb = false</pre></td>
      <td><code>#bbbbbb</code><br><pre>What OpenGL thinks: srgb (35904)
What glutin thinks: srgb = false</pre></td>
    </tr>
    <tr>
      <td>Windows 10</td>
      <td><code>#bdbdbd</code><br><pre>What OpenGL thinks: linear (9729)
What glutin thinks: srgb = true</pre></td>
      <td><code>#bdbdbd</code><br><pre>What OpenGL thinks: linear (9729)
What glutin thinks: srgb = true</pre></td>
    </tr>
  </tbody>
</table>


### Platform information
#### Ubuntu 18.04

<details>

```
$ lspci
00:00.0 Host bridge: Intel Corporation Xeon E3-1200 v5/E3-1500 v5/6th Gen Core Processor Host Bridge/DRAM Registers (rev 07)
00:01.0 PCI bridge: Intel Corporation Xeon E3-1200 v5/E3-1500 v5/6th Gen Core Processor PCIe Controller (x16) (rev 07)
00:01.2 PCI bridge: Intel Corporation Xeon E3-1200 v5/E3-1500 v5/6th Gen Core Processor PCIe Controller (x4) (rev 07)
00:02.0 VGA compatible controller: Intel Corporation HD Graphics 530 (rev 06)
00:14.0 USB controller: Intel Corporation 100 Series/C230 Series Chipset Family USB 3.0 xHCI Controller (rev 31)
00:14.2 Signal processing controller: Intel Corporation 100 Series/C230 Series Chipset Family Thermal Subsystem (rev 31)
00:16.0 Communication controller: Intel Corporation 100 Series/C230 Series Chipset Family MEI Controller #1 (rev 31)
00:17.0 SATA controller: Intel Corporation HM170/QM170 Chipset SATA Controller [AHCI Mode] (rev 31)
00:1c.0 PCI bridge: Intel Corporation 100 Series/C230 Series Chipset Family PCI Express Root Port #1 (rev f1)
00:1c.4 PCI bridge: Intel Corporation 100 Series/C230 Series Chipset Family PCI Express Root Port #5 (rev f1)
00:1f.0 ISA bridge: Intel Corporation QM170 Chipset LPC/eSPI Controller (rev 31)
00:1f.2 Memory controller: Intel Corporation 100 Series/C230 Series Chipset Family Power Management Controller (rev 31)
00:1f.3 Audio device: Intel Corporation 100 Series/C230 Series Chipset Family HD Audio Controller (rev 31)
00:1f.4 SMBus: Intel Corporation 100 Series/C230 Series Chipset Family SMBus (rev 31)
00:1f.6 Ethernet controller: Intel Corporation Ethernet Connection (2) I219-LM (rev 31)
02:00.0 3D controller: NVIDIA Corporation GM108M [GeForce 940MX] (rev a2)
03:00.0 Network controller: Intel Corporation Wireless 8260 (rev 3a)
04:00.0 Unassigned class [ff00]: Realtek Semiconductor Co., Ltd. RTS522A PCI Express Card Reader (rev 01)


$ glxinfo
name of display: :0
display: :0  screen: 0
direct rendering: Yes
server glx vendor string: SGI
server glx version string: 1.4
server glx extensions:
    GLX_ARB_create_context, GLX_ARB_create_context_profile, 
    GLX_ARB_create_context_robustness, GLX_ARB_fbconfig_float, 
    GLX_ARB_framebuffer_sRGB, GLX_ARB_multisample, 
    GLX_EXT_create_context_es2_profile, GLX_EXT_create_context_es_profile, 
    GLX_EXT_fbconfig_packed_float, GLX_EXT_framebuffer_sRGB, 
    GLX_EXT_import_context, GLX_EXT_libglvnd, GLX_EXT_texture_from_pixmap, 
    GLX_EXT_visual_info, GLX_EXT_visual_rating, GLX_INTEL_swap_event, 
    GLX_MESA_copy_sub_buffer, GLX_OML_swap_method, GLX_SGIS_multisample, 
    GLX_SGIX_fbconfig, GLX_SGIX_pbuffer, GLX_SGIX_visual_select_group, 
    GLX_SGI_make_current_read, GLX_SGI_swap_control
client glx vendor string: Mesa Project and SGI
client glx version string: 1.4
client glx extensions:
    GLX_ARB_context_flush_control, GLX_ARB_create_context, 
    GLX_ARB_create_context_profile, GLX_ARB_create_context_robustness, 
    GLX_ARB_fbconfig_float, GLX_ARB_framebuffer_sRGB, 
    GLX_ARB_get_proc_address, GLX_ARB_multisample, GLX_EXT_buffer_age, 
    GLX_EXT_create_context_es2_profile, GLX_EXT_create_context_es_profile, 
    GLX_EXT_fbconfig_packed_float, GLX_EXT_framebuffer_sRGB, 
    GLX_EXT_import_context, GLX_EXT_texture_from_pixmap, GLX_EXT_visual_info, 
    GLX_EXT_visual_rating, GLX_INTEL_swap_event, GLX_MESA_copy_sub_buffer, 
    GLX_MESA_multithread_makecurrent, GLX_MESA_query_renderer, 
    GLX_MESA_swap_control, GLX_OML_swap_method, GLX_OML_sync_control, 
    GLX_SGIS_multisample, GLX_SGIX_fbconfig, GLX_SGIX_pbuffer, 
    GLX_SGIX_visual_select_group, GLX_SGI_make_current_read, 
    GLX_SGI_swap_control, GLX_SGI_video_sync
GLX version: 1.4
GLX extensions:
    GLX_ARB_create_context, GLX_ARB_create_context_profile, 
    GLX_ARB_create_context_robustness, GLX_ARB_fbconfig_float, 
    GLX_ARB_framebuffer_sRGB, GLX_ARB_get_proc_address, GLX_ARB_multisample, 
    GLX_EXT_buffer_age, GLX_EXT_create_context_es2_profile, 
    GLX_EXT_create_context_es_profile, GLX_EXT_fbconfig_packed_float, 
    GLX_EXT_framebuffer_sRGB, GLX_EXT_import_context, 
    GLX_EXT_texture_from_pixmap, GLX_EXT_visual_info, GLX_EXT_visual_rating, 
    GLX_INTEL_swap_event, GLX_MESA_copy_sub_buffer, GLX_MESA_query_renderer, 
    GLX_MESA_swap_control, GLX_OML_swap_method, GLX_OML_sync_control, 
    GLX_SGIS_multisample, GLX_SGIX_fbconfig, GLX_SGIX_pbuffer, 
    GLX_SGIX_visual_select_group, GLX_SGI_make_current_read, 
    GLX_SGI_swap_control, GLX_SGI_video_sync
Extended renderer info (GLX_MESA_query_renderer):
    Vendor: Intel Open Source Technology Center (0x8086)
    Device: Mesa DRI Intel(R) HD Graphics 530 (Skylake GT2)  (0x191b)
    Version: 18.2.8
    Accelerated: yes
    Video memory: 3072MB
    Unified memory: yes
    Preferred profile: core (0x1)
    Max core profile version: 4.5
    Max compat profile version: 3.0
    Max GLES1 profile version: 1.1
    Max GLES[23] profile version: 3.2
OpenGL vendor string: Intel Open Source Technology Center
OpenGL renderer string: Mesa DRI Intel(R) HD Graphics 530 (Skylake GT2) 
OpenGL core profile version string: 4.5 (Core Profile) Mesa 18.2.8
OpenGL core profile shading language version string: 4.50
OpenGL core profile context flags: (none)
OpenGL core profile profile mask: core profile
OpenGL core profile extensions:
    GL_3DFX_texture_compression_FXT1, GL_AMD_conservative_depth, 
    GL_AMD_draw_buffers_blend, GL_AMD_seamless_cubemap_per_texture, 
    GL_AMD_shader_stencil_export, GL_AMD_shader_trinary_minmax, 
    GL_AMD_vertex_shader_layer, GL_AMD_vertex_shader_viewport_index, 
    GL_ANGLE_texture_compression_dxt3, GL_ANGLE_texture_compression_dxt5, 
    GL_APPLE_object_purgeable, GL_ARB_ES2_compatibility, 
    GL_ARB_ES3_1_compatibility, GL_ARB_ES3_2_compatibility, 
    GL_ARB_ES3_compatibility, GL_ARB_arrays_of_arrays, GL_ARB_base_instance, 
    GL_ARB_blend_func_extended, GL_ARB_buffer_storage, 
    GL_ARB_clear_buffer_object, GL_ARB_clear_texture, GL_ARB_clip_control, 
    GL_ARB_compressed_texture_pixel_storage, GL_ARB_compute_shader, 
    GL_ARB_conditional_render_inverted, GL_ARB_conservative_depth, 
    GL_ARB_copy_buffer, GL_ARB_copy_image, GL_ARB_cull_distance, 
    GL_ARB_debug_output, GL_ARB_depth_buffer_float, GL_ARB_depth_clamp, 
    GL_ARB_derivative_control, GL_ARB_direct_state_access, 
    GL_ARB_draw_buffers, GL_ARB_draw_buffers_blend, 
    GL_ARB_draw_elements_base_vertex, GL_ARB_draw_indirect, 
    GL_ARB_draw_instanced, GL_ARB_enhanced_layouts, 
    GL_ARB_explicit_attrib_location, GL_ARB_explicit_uniform_location, 
    GL_ARB_fragment_coord_conventions, GL_ARB_fragment_layer_viewport, 
    GL_ARB_fragment_shader, GL_ARB_fragment_shader_interlock, 
    GL_ARB_framebuffer_no_attachments, GL_ARB_framebuffer_object, 
    GL_ARB_framebuffer_sRGB, GL_ARB_get_program_binary, 
    GL_ARB_get_texture_sub_image, GL_ARB_gpu_shader5, GL_ARB_gpu_shader_fp64, 
    GL_ARB_gpu_shader_int64, GL_ARB_half_float_pixel, 
    GL_ARB_half_float_vertex, GL_ARB_indirect_parameters, 
    GL_ARB_instanced_arrays, GL_ARB_internalformat_query, 
    GL_ARB_internalformat_query2, GL_ARB_invalidate_subdata, 
    GL_ARB_map_buffer_alignment, GL_ARB_map_buffer_range, GL_ARB_multi_bind, 
    GL_ARB_multi_draw_indirect, GL_ARB_occlusion_query2, 
    GL_ARB_pipeline_statistics_query, GL_ARB_pixel_buffer_object, 
    GL_ARB_point_sprite, GL_ARB_polygon_offset_clamp, 
    GL_ARB_post_depth_coverage, GL_ARB_program_interface_query, 
    GL_ARB_provoking_vertex, GL_ARB_query_buffer_object, 
    GL_ARB_robust_buffer_access_behavior, GL_ARB_robustness, 
    GL_ARB_sample_shading, GL_ARB_sampler_objects, GL_ARB_seamless_cube_map, 
    GL_ARB_seamless_cubemap_per_texture, GL_ARB_separate_shader_objects, 
    GL_ARB_shader_atomic_counter_ops, GL_ARB_shader_atomic_counters, 
    GL_ARB_shader_ballot, GL_ARB_shader_bit_encoding, GL_ARB_shader_clock, 
    GL_ARB_shader_draw_parameters, GL_ARB_shader_group_vote, 
    GL_ARB_shader_image_load_store, GL_ARB_shader_image_size, 
    GL_ARB_shader_objects, GL_ARB_shader_precision, 
    GL_ARB_shader_stencil_export, GL_ARB_shader_storage_buffer_object, 
    GL_ARB_shader_subroutine, GL_ARB_shader_texture_image_samples, 
    GL_ARB_shader_texture_lod, GL_ARB_shader_viewport_layer_array, 
    GL_ARB_shading_language_420pack, GL_ARB_shading_language_packing, 
    GL_ARB_stencil_texturing, GL_ARB_sync, GL_ARB_tessellation_shader, 
    GL_ARB_texture_barrier, GL_ARB_texture_buffer_object, 
    GL_ARB_texture_buffer_object_rgb32, GL_ARB_texture_buffer_range, 
    GL_ARB_texture_compression_bptc, GL_ARB_texture_compression_rgtc, 
    GL_ARB_texture_cube_map_array, GL_ARB_texture_filter_anisotropic, 
    GL_ARB_texture_float, GL_ARB_texture_gather, 
    GL_ARB_texture_mirror_clamp_to_edge, GL_ARB_texture_multisample, 
    GL_ARB_texture_non_power_of_two, GL_ARB_texture_query_levels, 
    GL_ARB_texture_query_lod, GL_ARB_texture_rectangle, GL_ARB_texture_rg, 
    GL_ARB_texture_rgb10_a2ui, GL_ARB_texture_stencil8, 
    GL_ARB_texture_storage, GL_ARB_texture_storage_multisample, 
    GL_ARB_texture_swizzle, GL_ARB_texture_view, GL_ARB_timer_query, 
    GL_ARB_transform_feedback2, GL_ARB_transform_feedback3, 
    GL_ARB_transform_feedback_instanced, 
    GL_ARB_transform_feedback_overflow_query, GL_ARB_uniform_buffer_object, 
    GL_ARB_vertex_array_bgra, GL_ARB_vertex_array_object, 
    GL_ARB_vertex_attrib_64bit, GL_ARB_vertex_attrib_binding, 
    GL_ARB_vertex_shader, GL_ARB_vertex_type_10f_11f_11f_rev, 
    GL_ARB_vertex_type_2_10_10_10_rev, GL_ARB_viewport_array, 
    GL_ATI_blend_equation_separate, GL_ATI_texture_float, GL_EXT_abgr, 
    GL_EXT_blend_equation_separate, GL_EXT_draw_buffers2, 
    GL_EXT_draw_instanced, GL_EXT_framebuffer_blit, 
    GL_EXT_framebuffer_multisample, GL_EXT_framebuffer_multisample_blit_scaled, 
    GL_EXT_framebuffer_sRGB, GL_EXT_packed_depth_stencil, GL_EXT_packed_float, 
    GL_EXT_pixel_buffer_object, GL_EXT_polygon_offset_clamp, 
    GL_EXT_provoking_vertex, GL_EXT_shader_framebuffer_fetch, 
    GL_EXT_shader_framebuffer_fetch_non_coherent, GL_EXT_shader_integer_mix, 
    GL_EXT_shader_samples_identical, GL_EXT_texture_array, 
    GL_EXT_texture_compression_dxt1, GL_EXT_texture_compression_rgtc, 
    GL_EXT_texture_compression_s3tc, GL_EXT_texture_filter_anisotropic, 
    GL_EXT_texture_integer, GL_EXT_texture_sRGB, GL_EXT_texture_sRGB_decode, 
    GL_EXT_texture_shared_exponent, GL_EXT_texture_snorm, 
    GL_EXT_texture_swizzle, GL_EXT_timer_query, GL_EXT_transform_feedback, 
    GL_EXT_vertex_array_bgra, GL_IBM_multimode_draw_arrays, 
    GL_INTEL_conservative_rasterization, GL_INTEL_performance_query, 
    GL_KHR_blend_equation_advanced, GL_KHR_blend_equation_advanced_coherent, 
    GL_KHR_context_flush_control, GL_KHR_debug, GL_KHR_no_error, 
    GL_KHR_robust_buffer_access_behavior, GL_KHR_robustness, 
    GL_KHR_texture_compression_astc_ldr, 
    GL_KHR_texture_compression_astc_sliced_3d, GL_MESA_pack_invert, 
    GL_MESA_shader_integer_functions, GL_MESA_texture_signed_rgba, 
    GL_NV_conditional_render, GL_NV_depth_clamp, GL_NV_packed_depth_stencil, 
    GL_NV_texture_barrier, GL_OES_EGL_image, GL_S3_s3tc

OpenGL version string: 3.0 Mesa 18.2.8
OpenGL shading language version string: 1.30
OpenGL context flags: (none)
OpenGL extensions:
    GL_3DFX_texture_compression_FXT1, GL_AMD_conservative_depth, 
    GL_AMD_draw_buffers_blend, GL_AMD_seamless_cubemap_per_texture, 
    GL_AMD_shader_stencil_export, GL_AMD_shader_trinary_minmax, 
    GL_ANGLE_texture_compression_dxt3, GL_ANGLE_texture_compression_dxt5, 
    GL_APPLE_object_purgeable, GL_APPLE_packed_pixels, 
    GL_ARB_ES2_compatibility, GL_ARB_ES3_1_compatibility, 
    GL_ARB_ES3_compatibility, GL_ARB_arrays_of_arrays, 
    GL_ARB_blend_func_extended, GL_ARB_buffer_storage, 
    GL_ARB_clear_buffer_object, GL_ARB_clear_texture, GL_ARB_clip_control, 
    GL_ARB_color_buffer_float, GL_ARB_compressed_texture_pixel_storage, 
    GL_ARB_compute_shader, GL_ARB_conditional_render_inverted, 
    GL_ARB_conservative_depth, GL_ARB_copy_buffer, GL_ARB_copy_image, 
    GL_ARB_cull_distance, GL_ARB_debug_output, GL_ARB_depth_buffer_float, 
    GL_ARB_depth_clamp, GL_ARB_depth_texture, GL_ARB_derivative_control, 
    GL_ARB_draw_buffers, GL_ARB_draw_buffers_blend, 
    GL_ARB_draw_elements_base_vertex, GL_ARB_draw_indirect, 
    GL_ARB_draw_instanced, GL_ARB_explicit_attrib_location, 
    GL_ARB_explicit_uniform_location, GL_ARB_fragment_coord_conventions, 
    GL_ARB_fragment_layer_viewport, GL_ARB_fragment_program, 
    GL_ARB_fragment_program_shadow, GL_ARB_fragment_shader, 
    GL_ARB_fragment_shader_interlock, GL_ARB_framebuffer_no_attachments, 
    GL_ARB_framebuffer_object, GL_ARB_framebuffer_sRGB, 
    GL_ARB_get_program_binary, GL_ARB_get_texture_sub_image, 
    GL_ARB_half_float_pixel, GL_ARB_half_float_vertex, 
    GL_ARB_indirect_parameters, GL_ARB_instanced_arrays, 
    GL_ARB_internalformat_query, GL_ARB_internalformat_query2, 
    GL_ARB_invalidate_subdata, GL_ARB_map_buffer_alignment, 
    GL_ARB_map_buffer_range, GL_ARB_multi_bind, GL_ARB_multi_draw_indirect, 
    GL_ARB_multisample, GL_ARB_multitexture, GL_ARB_occlusion_query, 
    GL_ARB_occlusion_query2, GL_ARB_pipeline_statistics_query, 
    GL_ARB_pixel_buffer_object, GL_ARB_point_parameters, GL_ARB_point_sprite, 
    GL_ARB_polygon_offset_clamp, GL_ARB_program_interface_query, 
    GL_ARB_provoking_vertex, GL_ARB_query_buffer_object, 
    GL_ARB_robust_buffer_access_behavior, GL_ARB_robustness, 
    GL_ARB_sample_shading, GL_ARB_sampler_objects, GL_ARB_seamless_cube_map, 
    GL_ARB_seamless_cubemap_per_texture, GL_ARB_separate_shader_objects, 
    GL_ARB_shader_atomic_counter_ops, GL_ARB_shader_atomic_counters, 
    GL_ARB_shader_ballot, GL_ARB_shader_bit_encoding, GL_ARB_shader_clock, 
    GL_ARB_shader_draw_parameters, GL_ARB_shader_group_vote, 
    GL_ARB_shader_image_load_store, GL_ARB_shader_image_size, 
    GL_ARB_shader_objects, GL_ARB_shader_precision, 
    GL_ARB_shader_stencil_export, GL_ARB_shader_storage_buffer_object, 
    GL_ARB_shader_texture_image_samples, GL_ARB_shader_texture_lod, 
    GL_ARB_shading_language_100, GL_ARB_shading_language_420pack, 
    GL_ARB_shading_language_packing, GL_ARB_shadow, GL_ARB_stencil_texturing, 
    GL_ARB_sync, GL_ARB_texture_barrier, GL_ARB_texture_border_clamp, 
    GL_ARB_texture_compression, GL_ARB_texture_compression_bptc, 
    GL_ARB_texture_compression_rgtc, GL_ARB_texture_cube_map, 
    GL_ARB_texture_cube_map_array, GL_ARB_texture_env_add, 
    GL_ARB_texture_env_combine, GL_ARB_texture_env_crossbar, 
    GL_ARB_texture_env_dot3, GL_ARB_texture_filter_anisotropic, 
    GL_ARB_texture_float, GL_ARB_texture_gather, 
    GL_ARB_texture_mirror_clamp_to_edge, GL_ARB_texture_mirrored_repeat, 
    GL_ARB_texture_multisample, GL_ARB_texture_non_power_of_two, 
    GL_ARB_texture_query_levels, GL_ARB_texture_query_lod, 
    GL_ARB_texture_rectangle, GL_ARB_texture_rg, GL_ARB_texture_rgb10_a2ui, 
    GL_ARB_texture_stencil8, GL_ARB_texture_storage, 
    GL_ARB_texture_storage_multisample, GL_ARB_texture_swizzle, 
    GL_ARB_texture_view, GL_ARB_timer_query, GL_ARB_transform_feedback2, 
    GL_ARB_transform_feedback3, GL_ARB_transform_feedback_instanced, 
    GL_ARB_transform_feedback_overflow_query, GL_ARB_transpose_matrix, 
    GL_ARB_uniform_buffer_object, GL_ARB_vertex_array_bgra, 
    GL_ARB_vertex_array_object, GL_ARB_vertex_attrib_binding, 
    GL_ARB_vertex_buffer_object, GL_ARB_vertex_program, GL_ARB_vertex_shader, 
    GL_ARB_vertex_type_10f_11f_11f_rev, GL_ARB_vertex_type_2_10_10_10_rev, 
    GL_ARB_window_pos, GL_ATI_blend_equation_separate, GL_ATI_draw_buffers, 
    GL_ATI_separate_stencil, GL_ATI_texture_env_combine3, 
    GL_ATI_texture_float, GL_EXT_abgr, GL_EXT_bgra, GL_EXT_blend_color, 
    GL_EXT_blend_equation_separate, GL_EXT_blend_func_separate, 
    GL_EXT_blend_minmax, GL_EXT_blend_subtract, GL_EXT_compiled_vertex_array, 
    GL_EXT_copy_texture, GL_EXT_draw_buffers2, GL_EXT_draw_instanced, 
    GL_EXT_draw_range_elements, GL_EXT_fog_coord, GL_EXT_framebuffer_blit, 
    GL_EXT_framebuffer_multisample, GL_EXT_framebuffer_multisample_blit_scaled, 
    GL_EXT_framebuffer_object, GL_EXT_framebuffer_sRGB, 
    GL_EXT_gpu_program_parameters, GL_EXT_multi_draw_arrays, 
    GL_EXT_packed_depth_stencil, GL_EXT_packed_float, GL_EXT_packed_pixels, 
    GL_EXT_pixel_buffer_object, GL_EXT_point_parameters, 
    GL_EXT_polygon_offset_clamp, GL_EXT_provoking_vertex, 
    GL_EXT_rescale_normal, GL_EXT_secondary_color, 
    GL_EXT_separate_specular_color, GL_EXT_shader_framebuffer_fetch, 
    GL_EXT_shader_framebuffer_fetch_non_coherent, GL_EXT_shader_integer_mix, 
    GL_EXT_shader_samples_identical, GL_EXT_shadow_funcs, 
    GL_EXT_stencil_two_side, GL_EXT_stencil_wrap, GL_EXT_subtexture, 
    GL_EXT_texture, GL_EXT_texture3D, GL_EXT_texture_array, 
    GL_EXT_texture_compression_dxt1, GL_EXT_texture_compression_rgtc, 
    GL_EXT_texture_compression_s3tc, GL_EXT_texture_cube_map, 
    GL_EXT_texture_edge_clamp, GL_EXT_texture_env_add, 
    GL_EXT_texture_env_combine, GL_EXT_texture_env_dot3, 
    GL_EXT_texture_filter_anisotropic, GL_EXT_texture_integer, 
    GL_EXT_texture_lod_bias, GL_EXT_texture_object, GL_EXT_texture_rectangle, 
    GL_EXT_texture_sRGB, GL_EXT_texture_sRGB_decode, 
    GL_EXT_texture_shared_exponent, GL_EXT_texture_snorm, 
    GL_EXT_texture_swizzle, GL_EXT_timer_query, GL_EXT_transform_feedback, 
    GL_EXT_vertex_array, GL_EXT_vertex_array_bgra, 
    GL_IBM_multimode_draw_arrays, GL_IBM_rasterpos_clip, 
    GL_IBM_texture_mirrored_repeat, GL_INGR_blend_func_separate, 
    GL_INTEL_performance_query, GL_KHR_blend_equation_advanced, 
    GL_KHR_blend_equation_advanced_coherent, GL_KHR_context_flush_control, 
    GL_KHR_debug, GL_KHR_no_error, GL_KHR_robust_buffer_access_behavior, 
    GL_KHR_robustness, GL_KHR_texture_compression_astc_ldr, 
    GL_KHR_texture_compression_astc_sliced_3d, GL_MESA_pack_invert, 
    GL_MESA_shader_integer_functions, GL_MESA_texture_signed_rgba, 
    GL_MESA_window_pos, GL_NV_blend_square, GL_NV_conditional_render, 
    GL_NV_depth_clamp, GL_NV_fog_distance, GL_NV_light_max_exponent, 
    GL_NV_packed_depth_stencil, GL_NV_primitive_restart, 
    GL_NV_texgen_reflection, GL_NV_texture_barrier, 
    GL_NV_texture_env_combine4, GL_NV_texture_rectangle, GL_OES_EGL_image, 
    GL_OES_read_format, GL_S3_s3tc, GL_SGIS_generate_mipmap, 
    GL_SGIS_texture_border_clamp, GL_SGIS_texture_edge_clamp, 
    GL_SGIS_texture_lod, GL_SUN_multi_draw_arrays

OpenGL ES profile version string: OpenGL ES 3.2 Mesa 18.2.8
OpenGL ES profile shading language version string: OpenGL ES GLSL ES 3.20
OpenGL ES profile extensions:
    GL_ANDROID_extension_pack_es31a, GL_ANGLE_texture_compression_dxt3, 
    GL_ANGLE_texture_compression_dxt5, GL_APPLE_texture_max_level, 
    GL_EXT_base_instance, GL_EXT_blend_func_extended, GL_EXT_blend_minmax, 
    GL_EXT_buffer_storage, GL_EXT_clip_cull_distance, 
    GL_EXT_color_buffer_float, GL_EXT_compressed_ETC1_RGB8_sub_texture, 
    GL_EXT_copy_image, GL_EXT_discard_framebuffer, 
    GL_EXT_disjoint_timer_query, GL_EXT_draw_buffers, 
    GL_EXT_draw_buffers_indexed, GL_EXT_draw_elements_base_vertex, 
    GL_EXT_frag_depth, GL_EXT_geometry_point_size, GL_EXT_geometry_shader, 
    GL_EXT_gpu_shader5, GL_EXT_map_buffer_range, GL_EXT_multi_draw_arrays, 
    GL_EXT_occlusion_query_boolean, GL_EXT_polygon_offset_clamp, 
    GL_EXT_primitive_bounding_box, GL_EXT_read_format_bgra, GL_EXT_robustness, 
    GL_EXT_separate_shader_objects, GL_EXT_shader_framebuffer_fetch, 
    GL_EXT_shader_framebuffer_fetch_non_coherent, GL_EXT_shader_integer_mix, 
    GL_EXT_shader_io_blocks, GL_EXT_shader_samples_identical, 
    GL_EXT_tessellation_point_size, GL_EXT_tessellation_shader, 
    GL_EXT_texture_border_clamp, GL_EXT_texture_buffer, 
    GL_EXT_texture_compression_dxt1, GL_EXT_texture_cube_map_array, 
    GL_EXT_texture_filter_anisotropic, GL_EXT_texture_format_BGRA8888, 
    GL_EXT_texture_norm16, GL_EXT_texture_rg, GL_EXT_texture_sRGB_decode, 
    GL_EXT_texture_type_2_10_10_10_REV, GL_EXT_unpack_subimage, 
    GL_INTEL_conservative_rasterization, GL_INTEL_performance_query, 
    GL_KHR_blend_equation_advanced, GL_KHR_blend_equation_advanced_coherent, 
    GL_KHR_context_flush_control, GL_KHR_debug, GL_KHR_no_error, 
    GL_KHR_robust_buffer_access_behavior, GL_KHR_robustness, 
    GL_KHR_texture_compression_astc_ldr, 
    GL_KHR_texture_compression_astc_sliced_3d, GL_MESA_framebuffer_flip_y, 
    GL_MESA_shader_integer_functions, GL_NV_draw_buffers, 
    GL_NV_fbo_color_attachments, GL_NV_image_formats, GL_NV_read_buffer, 
    GL_NV_read_depth, GL_NV_read_depth_stencil, GL_NV_read_stencil, 
    GL_OES_EGL_image, GL_OES_EGL_image_external, 
    GL_OES_EGL_image_external_essl3, GL_OES_EGL_sync, 
    GL_OES_compressed_ETC1_RGB8_texture, GL_OES_copy_image, GL_OES_depth24, 
    GL_OES_depth_texture, GL_OES_depth_texture_cube_map, 
    GL_OES_draw_buffers_indexed, GL_OES_draw_elements_base_vertex, 
    GL_OES_element_index_uint, GL_OES_fbo_render_mipmap, 
    GL_OES_geometry_point_size, GL_OES_geometry_shader, 
    GL_OES_get_program_binary, GL_OES_gpu_shader5, GL_OES_mapbuffer, 
    GL_OES_packed_depth_stencil, GL_OES_primitive_bounding_box, 
    GL_OES_required_internalformat, GL_OES_rgb8_rgba8, GL_OES_sample_shading, 
    GL_OES_sample_variables, GL_OES_shader_image_atomic, 
    GL_OES_shader_io_blocks, GL_OES_shader_multisample_interpolation, 
    GL_OES_standard_derivatives, GL_OES_stencil8, GL_OES_surfaceless_context, 
    GL_OES_tessellation_point_size, GL_OES_tessellation_shader, 
    GL_OES_texture_3D, GL_OES_texture_border_clamp, GL_OES_texture_buffer, 
    GL_OES_texture_cube_map_array, GL_OES_texture_float, 
    GL_OES_texture_float_linear, GL_OES_texture_half_float, 
    GL_OES_texture_half_float_linear, GL_OES_texture_npot, 
    GL_OES_texture_stencil8, GL_OES_texture_storage_multisample_2d_array, 
    GL_OES_texture_view, GL_OES_vertex_array_object, GL_OES_vertex_half_float, 
    GL_OES_viewport_array

94 GLX Visuals
    visual  x   bf lv rg d st  colorbuffer  sr ax dp st accumbuffer  ms  cav
  id dep cl sp  sz l  ci b ro  r  g  b  a F gb bf th cl  r  g  b  a ns b eat
----------------------------------------------------------------------------
0x041 24 tc  0  32  0 r  y .   8  8  8  8 .  .  0 24  8  0  0  0  0  0 0 None
0x042 24 dc  0  32  0 r  y .   8  8  8  8 .  .  0 24  8  0  0  0  0  0 0 None
0x161 24 tc  0  32  0 r  y .   8  8  8  8 .  .  0  0  0  0  0  0  0  0 0 None
0x162 24 tc  0  32  0 r  . .   8  8  8  8 .  .  0  0  0  0  0  0  0  0 0 None
0x163 24 tc  0  32  0 r  . .   8  8  8  8 .  .  0 24  8  0  0  0  0  0 0 None
0x164 24 tc  0  24  0 r  y .   8  8  8  0 .  .  0  0  0  0  0  0  0  0 0 None
0x165 24 tc  0  24  0 r  . .   8  8  8  0 .  .  0  0  0  0  0  0  0  0 0 None
0x166 24 tc  0  24  0 r  y .   8  8  8  0 .  .  0 24  8  0  0  0  0  0 0 None
0x167 24 tc  0  24  0 r  . .   8  8  8  0 .  .  0 24  8  0  0  0  0  0 0 None
0x168 24 tc  0  32  0 r  y .   8  8  8  8 .  s  0  0  0  0  0  0  0  0 0 None
0x169 24 tc  0  32  0 r  . .   8  8  8  8 .  s  0  0  0  0  0  0  0  0 0 None
0x16a 24 tc  0  32  0 r  y .   8  8  8  8 .  s  0 24  8  0  0  0  0  0 0 None
0x16b 24 tc  0  32  0 r  . .   8  8  8  8 .  s  0 24  8  0  0  0  0  0 0 None
0x16c 24 tc  0  32  0 r  y .   8  8  8  8 .  .  0 24  8  0  0  0  0  0 0 None
0x16d 24 tc  0  32  0 r  y .   8  8  8  8 .  .  0 24  8 16 16 16 16  0 0 Slow
0x16e 24 tc  0  24  0 r  y .   8  8  8  0 .  .  0 24  8  0  0  0  0  0 0 None
0x16f 24 tc  0  24  0 r  y .   8  8  8  0 .  .  0 24  8 16 16 16  0  0 0 Slow
0x170 24 tc  0  32  0 r  y .   8  8  8  8 .  s  0 24  8  0  0  0  0  0 0 None
0x171 24 tc  0  32  0 r  y .   8  8  8  8 .  s  0 24  8 16 16 16 16  0 0 Slow
0x172 24 tc  0  32  0 r  y .   8  8  8  8 .  .  0  0  0  0  0  0  0  2 1 None
0x173 24 tc  0  32  0 r  y .   8  8  8  8 .  .  0  0  0  0  0  0  0  4 1 None
0x174 24 tc  0  32  0 r  y .   8  8  8  8 .  .  0  0  0  0  0  0  0  8 1 None
0x175 24 tc  0  32  0 r  y .   8  8  8  8 .  .  0  0  0  0  0  0  0 16 1 None
0x176 24 tc  0  32  0 r  y .   8  8  8  8 .  .  0 24  8  0  0  0  0  2 1 None
0x177 24 tc  0  32  0 r  y .   8  8  8  8 .  .  0 24  8  0  0  0  0  4 1 None
0x178 24 tc  0  32  0 r  y .   8  8  8  8 .  .  0 24  8  0  0  0  0  8 1 None
0x179 24 tc  0  32  0 r  y .   8  8  8  8 .  .  0 24  8  0  0  0  0 16 1 None
0x17a 24 tc  0  24  0 r  y .   8  8  8  0 .  .  0  0  0  0  0  0  0  2 1 None
0x17b 24 tc  0  24  0 r  y .   8  8  8  0 .  .  0  0  0  0  0  0  0  4 1 None
0x17c 24 tc  0  24  0 r  y .   8  8  8  0 .  .  0  0  0  0  0  0  0  8 1 None
0x17d 24 tc  0  24  0 r  y .   8  8  8  0 .  .  0  0  0  0  0  0  0 16 1 None
0x17e 24 tc  0  24  0 r  y .   8  8  8  0 .  .  0 24  8  0  0  0  0  2 1 None
0x17f 24 tc  0  24  0 r  y .   8  8  8  0 .  .  0 24  8  0  0  0  0  4 1 None
0x180 24 tc  0  24  0 r  y .   8  8  8  0 .  .  0 24  8  0  0  0  0  8 1 None
0x181 24 tc  0  24  0 r  y .   8  8  8  0 .  .  0 24  8  0  0  0  0 16 1 None
0x182 24 tc  0  32  0 r  y .   8  8  8  8 .  s  0  0  0  0  0  0  0  2 1 None
0x183 24 tc  0  32  0 r  y .   8  8  8  8 .  s  0  0  0  0  0  0  0  4 1 None
0x184 24 tc  0  32  0 r  y .   8  8  8  8 .  s  0  0  0  0  0  0  0  8 1 None
0x185 24 tc  0  32  0 r  y .   8  8  8  8 .  s  0  0  0  0  0  0  0 16 1 None
0x186 24 tc  0  32  0 r  y .   8  8  8  8 .  s  0 24  8  0  0  0  0  2 1 None
0x187 24 tc  0  32  0 r  y .   8  8  8  8 .  s  0 24  8  0  0  0  0  4 1 None
0x188 24 tc  0  32  0 r  y .   8  8  8  8 .  s  0 24  8  0  0  0  0  8 1 None
0x189 24 tc  0  32  0 r  y .   8  8  8  8 .  s  0 24  8  0  0  0  0 16 1 None
0x18a 24 dc  0  32  0 r  y .   8  8  8  8 .  .  0  0  0  0  0  0  0  0 0 None
0x18b 24 dc  0  32  0 r  . .   8  8  8  8 .  .  0  0  0  0  0  0  0  0 0 None
0x18c 24 dc  0  32  0 r  . .   8  8  8  8 .  .  0 24  8  0  0  0  0  0 0 None
0x18d 24 dc  0  24  0 r  y .   8  8  8  0 .  .  0  0  0  0  0  0  0  0 0 None
0x18e 24 dc  0  24  0 r  . .   8  8  8  0 .  .  0  0  0  0  0  0  0  0 0 None
0x18f 24 dc  0  24  0 r  y .   8  8  8  0 .  .  0 24  8  0  0  0  0  0 0 None
0x190 24 dc  0  24  0 r  . .   8  8  8  0 .  .  0 24  8  0  0  0  0  0 0 None
0x191 24 dc  0  32  0 r  y .   8  8  8  8 .  s  0  0  0  0  0  0  0  0 0 None
0x192 24 dc  0  32  0 r  . .   8  8  8  8 .  s  0  0  0  0  0  0  0  0 0 None
0x193 24 dc  0  32  0 r  y .   8  8  8  8 .  s  0 24  8  0  0  0  0  0 0 None
0x194 24 dc  0  32  0 r  . .   8  8  8  8 .  s  0 24  8  0  0  0  0  0 0 None
0x195 24 dc  0  32  0 r  y .   8  8  8  8 .  .  0 24  8  0  0  0  0  0 0 None
0x196 24 dc  0  32  0 r  y .   8  8  8  8 .  .  0 24  8 16 16 16 16  0 0 Slow
0x197 24 dc  0  24  0 r  y .   8  8  8  0 .  .  0 24  8  0  0  0  0  0 0 None
0x198 24 dc  0  24  0 r  y .   8  8  8  0 .  .  0 24  8 16 16 16  0  0 0 Slow
0x199 24 dc  0  32  0 r  y .   8  8  8  8 .  s  0 24  8  0  0  0  0  0 0 None
0x19a 24 dc  0  32  0 r  y .   8  8  8  8 .  s  0 24  8 16 16 16 16  0 0 Slow
0x19b 24 dc  0  32  0 r  y .   8  8  8  8 .  .  0  0  0  0  0  0  0  2 1 None
0x19c 24 dc  0  32  0 r  y .   8  8  8  8 .  .  0  0  0  0  0  0  0  4 1 None
0x19d 24 dc  0  32  0 r  y .   8  8  8  8 .  .  0  0  0  0  0  0  0  8 1 None
0x19e 24 dc  0  32  0 r  y .   8  8  8  8 .  .  0  0  0  0  0  0  0 16 1 None
0x19f 24 dc  0  32  0 r  y .   8  8  8  8 .  .  0 24  8  0  0  0  0  2 1 None
0x1a0 24 dc  0  32  0 r  y .   8  8  8  8 .  .  0 24  8  0  0  0  0  4 1 None
0x1a1 24 dc  0  32  0 r  y .   8  8  8  8 .  .  0 24  8  0  0  0  0  8 1 None
0x1a2 24 dc  0  32  0 r  y .   8  8  8  8 .  .  0 24  8  0  0  0  0 16 1 None
0x1a3 24 dc  0  24  0 r  y .   8  8  8  0 .  .  0  0  0  0  0  0  0  2 1 None
0x1a4 24 dc  0  24  0 r  y .   8  8  8  0 .  .  0  0  0  0  0  0  0  4 1 None
0x1a5 24 dc  0  24  0 r  y .   8  8  8  0 .  .  0  0  0  0  0  0  0  8 1 None
0x1a6 24 dc  0  24  0 r  y .   8  8  8  0 .  .  0  0  0  0  0  0  0 16 1 None
0x1a7 24 dc  0  24  0 r  y .   8  8  8  0 .  .  0 24  8  0  0  0  0  2 1 None
0x1a8 24 dc  0  24  0 r  y .   8  8  8  0 .  .  0 24  8  0  0  0  0  4 1 None
0x1a9 24 dc  0  24  0 r  y .   8  8  8  0 .  .  0 24  8  0  0  0  0  8 1 None
0x1aa 24 dc  0  24  0 r  y .   8  8  8  0 .  .  0 24  8  0  0  0  0 16 1 None
0x1ab 24 dc  0  32  0 r  y .   8  8  8  8 .  s  0  0  0  0  0  0  0  2 1 None
0x1ac 24 dc  0  32  0 r  y .   8  8  8  8 .  s  0  0  0  0  0  0  0  4 1 None
0x1ad 24 dc  0  32  0 r  y .   8  8  8  8 .  s  0  0  0  0  0  0  0  8 1 None
0x1ae 24 dc  0  32  0 r  y .   8  8  8  8 .  s  0  0  0  0  0  0  0 16 1 None
0x1af 24 dc  0  32  0 r  y .   8  8  8  8 .  s  0 24  8  0  0  0  0  2 1 None
0x1b0 24 dc  0  32  0 r  y .   8  8  8  8 .  s  0 24  8  0  0  0  0  4 1 None
0x1b1 24 dc  0  32  0 r  y .   8  8  8  8 .  s  0 24  8  0  0  0  0  8 1 None
0x1b2 24 dc  0  32  0 r  y .   8  8  8  8 .  s  0 24  8  0  0  0  0 16 1 None
0x0e2 32 tc  0  32  0 r  y .   8  8  8  8 .  .  0 24  8  0  0  0  0  0 0 None
0x1b3 32 tc  0  32  0 r  y .   8  8  8  8 .  .  0  0  0  0  0  0  0  0 0 None
0x1b4 32 tc  0  32  0 r  . .   8  8  8  8 .  .  0  0  0  0  0  0  0  0 0 None
0x1b5 32 tc  0  32  0 r  . .   8  8  8  8 .  .  0 24  8  0  0  0  0  0 0 None
0x1b6 32 tc  0  32  0 r  y .   8  8  8  8 .  s  0  0  0  0  0  0  0  0 0 None
0x1b7 32 tc  0  32  0 r  . .   8  8  8  8 .  s  0  0  0  0  0  0  0  0 0 None
0x1b8 32 tc  0  32  0 r  y .   8  8  8  8 .  s  0 24  8  0  0  0  0  0 0 None
0x1b9 32 tc  0  32  0 r  . .   8  8  8  8 .  s  0 24  8  0  0  0  0  0 0 None
0x1ba 32 tc  0  32  0 r  y .   8  8  8  8 .  .  0 24  8  0  0  0  0  0 0 None
0x1bb 32 tc  0  32  0 r  y .   8  8  8  8 .  s  0 24  8  0  0  0  0  0 0 None

126 GLXFBConfigs:
    visual  x   bf lv rg d st  colorbuffer  sr ax dp st accumbuffer  ms  cav
  id dep cl sp  sz l  ci b ro  r  g  b  a F gb bf th cl  r  g  b  a ns b eat
----------------------------------------------------------------------------
0x0e3  0 tc  0  16  0 r  y .   5  6  5  0 .  .  0  0  0  0  0  0  0  0 0 None
0x0e4  0 tc  0  16  0 r  . .   5  6  5  0 .  .  0  0  0  0  0  0  0  0 0 None
0x0e5  0 tc  0  16  0 r  y .   5  6  5  0 .  .  0 16  0  0  0  0  0  0 0 None
0x0e6  0 tc  0  16  0 r  . .   5  6  5  0 .  .  0 16  0  0  0  0  0  0 0 None
0x0e7  0 tc  0  16  0 r  y .   5  6  5  0 .  .  0 24  8  0  0  0  0  0 0 None
0x0e8  0 tc  0  16  0 r  . .   5  6  5  0 .  .  0 24  8  0  0  0  0  0 0 None
0x0e9 24 tc  0  32  0 r  y .   8  8  8  8 .  .  0  0  0  0  0  0  0  0 0 None
0x0ea 24 tc  0  32  0 r  . .   8  8  8  8 .  .  0  0  0  0  0  0  0  0 0 None
0x0eb 24 tc  0  32  0 r  y .   8  8  8  8 .  .  0 24  8  0  0  0  0  0 0 None
0x0ec 24 tc  0  32  0 r  . .   8  8  8  8 .  .  0 24  8  0  0  0  0  0 0 None
0x0ed 24 tc  0  24  0 r  y .   8  8  8  0 .  .  0  0  0  0  0  0  0  0 0 None
0x0ee 24 tc  0  24  0 r  . .   8  8  8  0 .  .  0  0  0  0  0  0  0  0 0 None
0x0ef 24 tc  0  24  0 r  y .   8  8  8  0 .  .  0 24  8  0  0  0  0  0 0 None
0x0f0 24 tc  0  24  0 r  . .   8  8  8  0 .  .  0 24  8  0  0  0  0  0 0 None
0x0f1 24 tc  0  32  0 r  y .   8  8  8  8 .  s  0  0  0  0  0  0  0  0 0 None
0x0f2 24 tc  0  32  0 r  . .   8  8  8  8 .  s  0  0  0  0  0  0  0  0 0 None
0x0f3 24 tc  0  32  0 r  y .   8  8  8  8 .  s  0 24  8  0  0  0  0  0 0 None
0x0f4 24 tc  0  32  0 r  . .   8  8  8  8 .  s  0 24  8  0  0  0  0  0 0 None
0x0f5  0 tc  0  16  0 r  y .   5  6  5  0 .  .  0 16  0  0  0  0  0  0 0 None
0x0f6  0 tc  0  16  0 r  y .   5  6  5  0 .  .  0 16  0 16 16 16  0  0 0 Slow
0x0f7 24 tc  0  32  0 r  y .   8  8  8  8 .  .  0 24  8  0  0  0  0  0 0 None
0x0f8 24 tc  0  32  0 r  y .   8  8  8  8 .  .  0 24  8 16 16 16 16  0 0 Slow
0x0f9 24 tc  0  24  0 r  y .   8  8  8  0 .  .  0 24  8  0  0  0  0  0 0 None
0x0fa 24 tc  0  24  0 r  y .   8  8  8  0 .  .  0 24  8 16 16 16  0  0 0 Slow
0x0fb 24 tc  0  32  0 r  y .   8  8  8  8 .  s  0 24  8  0  0  0  0  0 0 None
0x0fc 24 tc  0  32  0 r  y .   8  8  8  8 .  s  0 24  8 16 16 16 16  0 0 Slow
0x0fd  0 tc  0  16  0 r  y .   5  6  5  0 .  .  0  0  0  0  0  0  0  2 1 None
0x0fe  0 tc  0  16  0 r  y .   5  6  5  0 .  .  0  0  0  0  0  0  0  4 1 None
0x0ff  0 tc  0  16  0 r  y .   5  6  5  0 .  .  0  0  0  0  0  0  0  8 1 None
0x100  0 tc  0  16  0 r  y .   5  6  5  0 .  .  0  0  0  0  0  0  0 16 1 None
0x101  0 tc  0  16  0 r  y .   5  6  5  0 .  .  0 16  0  0  0  0  0  2 1 None
0x102  0 tc  0  16  0 r  y .   5  6  5  0 .  .  0 16  0  0  0  0  0  4 1 None
0x103  0 tc  0  16  0 r  y .   5  6  5  0 .  .  0 16  0  0  0  0  0  8 1 None
0x104  0 tc  0  16  0 r  y .   5  6  5  0 .  .  0 16  0  0  0  0  0 16 1 None
0x105 24 tc  0  32  0 r  y .   8  8  8  8 .  .  0  0  0  0  0  0  0  2 1 None
0x106 24 tc  0  32  0 r  y .   8  8  8  8 .  .  0  0  0  0  0  0  0  4 1 None
0x107 24 tc  0  32  0 r  y .   8  8  8  8 .  .  0  0  0  0  0  0  0  8 1 None
0x108 24 tc  0  32  0 r  y .   8  8  8  8 .  .  0  0  0  0  0  0  0 16 1 None
0x109 24 tc  0  32  0 r  y .   8  8  8  8 .  .  0 24  8  0  0  0  0  2 1 None
0x10a 24 tc  0  32  0 r  y .   8  8  8  8 .  .  0 24  8  0  0  0  0  4 1 None
0x10b 24 tc  0  32  0 r  y .   8  8  8  8 .  .  0 24  8  0  0  0  0  8 1 None
0x10c 24 tc  0  32  0 r  y .   8  8  8  8 .  .  0 24  8  0  0  0  0 16 1 None
0x10d 24 tc  0  24  0 r  y .   8  8  8  0 .  .  0  0  0  0  0  0  0  2 1 None
0x10e 24 tc  0  24  0 r  y .   8  8  8  0 .  .  0  0  0  0  0  0  0  4 1 None
0x10f 24 tc  0  24  0 r  y .   8  8  8  0 .  .  0  0  0  0  0  0  0  8 1 None
0x110 24 tc  0  24  0 r  y .   8  8  8  0 .  .  0  0  0  0  0  0  0 16 1 None
0x111 24 tc  0  24  0 r  y .   8  8  8  0 .  .  0 24  8  0  0  0  0  2 1 None
0x112 24 tc  0  24  0 r  y .   8  8  8  0 .  .  0 24  8  0  0  0  0  4 1 None
0x113 24 tc  0  24  0 r  y .   8  8  8  0 .  .  0 24  8  0  0  0  0  8 1 None
0x114 24 tc  0  24  0 r  y .   8  8  8  0 .  .  0 24  8  0  0  0  0 16 1 None
0x115 24 tc  0  32  0 r  y .   8  8  8  8 .  s  0  0  0  0  0  0  0  2 1 None
0x116 24 tc  0  32  0 r  y .   8  8  8  8 .  s  0  0  0  0  0  0  0  4 1 None
0x117 24 tc  0  32  0 r  y .   8  8  8  8 .  s  0  0  0  0  0  0  0  8 1 None
0x118 24 tc  0  32  0 r  y .   8  8  8  8 .  s  0  0  0  0  0  0  0 16 1 None
0x119 24 tc  0  32  0 r  y .   8  8  8  8 .  s  0 24  8  0  0  0  0  2 1 None
0x11a 24 tc  0  32  0 r  y .   8  8  8  8 .  s  0 24  8  0  0  0  0  4 1 None
0x11b 24 tc  0  32  0 r  y .   8  8  8  8 .  s  0 24  8  0  0  0  0  8 1 None
0x11c 24 tc  0  32  0 r  y .   8  8  8  8 .  s  0 24  8  0  0  0  0 16 1 None
0x11d  0 dc  0  16  0 r  y .   5  6  5  0 .  .  0  0  0  0  0  0  0  0 0 None
0x11e  0 dc  0  16  0 r  . .   5  6  5  0 .  .  0  0  0  0  0  0  0  0 0 None
0x11f  0 dc  0  16  0 r  y .   5  6  5  0 .  .  0 16  0  0  0  0  0  0 0 None
0x120  0 dc  0  16  0 r  . .   5  6  5  0 .  .  0 16  0  0  0  0  0  0 0 None
0x121  0 dc  0  16  0 r  y .   5  6  5  0 .  .  0 24  8  0  0  0  0  0 0 None
0x122  0 dc  0  16  0 r  . .   5  6  5  0 .  .  0 24  8  0  0  0  0  0 0 None
0x123 24 dc  0  32  0 r  y .   8  8  8  8 .  .  0  0  0  0  0  0  0  0 0 None
0x124 24 dc  0  32  0 r  . .   8  8  8  8 .  .  0  0  0  0  0  0  0  0 0 None
0x125 24 dc  0  32  0 r  y .   8  8  8  8 .  .  0 24  8  0  0  0  0  0 0 None
0x126 24 dc  0  32  0 r  . .   8  8  8  8 .  .  0 24  8  0  0  0  0  0 0 None
0x127 24 dc  0  24  0 r  y .   8  8  8  0 .  .  0  0  0  0  0  0  0  0 0 None
0x128 24 dc  0  24  0 r  . .   8  8  8  0 .  .  0  0  0  0  0  0  0  0 0 None
0x129 24 dc  0  24  0 r  y .   8  8  8  0 .  .  0 24  8  0  0  0  0  0 0 None
0x12a 24 dc  0  24  0 r  . .   8  8  8  0 .  .  0 24  8  0  0  0  0  0 0 None
0x12b 24 dc  0  32  0 r  y .   8  8  8  8 .  s  0  0  0  0  0  0  0  0 0 None
0x12c 24 dc  0  32  0 r  . .   8  8  8  8 .  s  0  0  0  0  0  0  0  0 0 None
0x12d 24 dc  0  32  0 r  y .   8  8  8  8 .  s  0 24  8  0  0  0  0  0 0 None
0x12e 24 dc  0  32  0 r  . .   8  8  8  8 .  s  0 24  8  0  0  0  0  0 0 None
0x12f  0 dc  0  16  0 r  y .   5  6  5  0 .  .  0 16  0  0  0  0  0  0 0 None
0x130  0 dc  0  16  0 r  y .   5  6  5  0 .  .  0 16  0 16 16 16  0  0 0 Slow
0x131 24 dc  0  32  0 r  y .   8  8  8  8 .  .  0 24  8  0  0  0  0  0 0 None
0x132 24 dc  0  32  0 r  y .   8  8  8  8 .  .  0 24  8 16 16 16 16  0 0 Slow
0x133 24 dc  0  24  0 r  y .   8  8  8  0 .  .  0 24  8  0  0  0  0  0 0 None
0x134 24 dc  0  24  0 r  y .   8  8  8  0 .  .  0 24  8 16 16 16  0  0 0 Slow
0x135 24 dc  0  32  0 r  y .   8  8  8  8 .  s  0 24  8  0  0  0  0  0 0 None
0x136 24 dc  0  32  0 r  y .   8  8  8  8 .  s  0 24  8 16 16 16 16  0 0 Slow
0x137  0 dc  0  16  0 r  y .   5  6  5  0 .  .  0  0  0  0  0  0  0  2 1 None
0x138  0 dc  0  16  0 r  y .   5  6  5  0 .  .  0  0  0  0  0  0  0  4 1 None
0x139  0 dc  0  16  0 r  y .   5  6  5  0 .  .  0  0  0  0  0  0  0  8 1 None
0x13a  0 dc  0  16  0 r  y .   5  6  5  0 .  .  0  0  0  0  0  0  0 16 1 None
0x13b  0 dc  0  16  0 r  y .   5  6  5  0 .  .  0 16  0  0  0  0  0  2 1 None
0x13c  0 dc  0  16  0 r  y .   5  6  5  0 .  .  0 16  0  0  0  0  0  4 1 None
0x13d  0 dc  0  16  0 r  y .   5  6  5  0 .  .  0 16  0  0  0  0  0  8 1 None
0x13e  0 dc  0  16  0 r  y .   5  6  5  0 .  .  0 16  0  0  0  0  0 16 1 None
0x13f 24 dc  0  32  0 r  y .   8  8  8  8 .  .  0  0  0  0  0  0  0  2 1 None
0x140 24 dc  0  32  0 r  y .   8  8  8  8 .  .  0  0  0  0  0  0  0  4 1 None
0x141 24 dc  0  32  0 r  y .   8  8  8  8 .  .  0  0  0  0  0  0  0  8 1 None
0x142 24 dc  0  32  0 r  y .   8  8  8  8 .  .  0  0  0  0  0  0  0 16 1 None
0x143 24 dc  0  32  0 r  y .   8  8  8  8 .  .  0 24  8  0  0  0  0  2 1 None
0x144 24 dc  0  32  0 r  y .   8  8  8  8 .  .  0 24  8  0  0  0  0  4 1 None
0x145 24 dc  0  32  0 r  y .   8  8  8  8 .  .  0 24  8  0  0  0  0  8 1 None
0x146 24 dc  0  32  0 r  y .   8  8  8  8 .  .  0 24  8  0  0  0  0 16 1 None
0x147 24 dc  0  24  0 r  y .   8  8  8  0 .  .  0  0  0  0  0  0  0  2 1 None
0x148 24 dc  0  24  0 r  y .   8  8  8  0 .  .  0  0  0  0  0  0  0  4 1 None
0x149 24 dc  0  24  0 r  y .   8  8  8  0 .  .  0  0  0  0  0  0  0  8 1 None
0x14a 24 dc  0  24  0 r  y .   8  8  8  0 .  .  0  0  0  0  0  0  0 16 1 None
0x14b 24 dc  0  24  0 r  y .   8  8  8  0 .  .  0 24  8  0  0  0  0  2 1 None
0x14c 24 dc  0  24  0 r  y .   8  8  8  0 .  .  0 24  8  0  0  0  0  4 1 None
0x14d 24 dc  0  24  0 r  y .   8  8  8  0 .  .  0 24  8  0  0  0  0  8 1 None
0x14e 24 dc  0  24  0 r  y .   8  8  8  0 .  .  0 24  8  0  0  0  0 16 1 None
0x14f 24 dc  0  32  0 r  y .   8  8  8  8 .  s  0  0  0  0  0  0  0  2 1 None
0x150 24 dc  0  32  0 r  y .   8  8  8  8 .  s  0  0  0  0  0  0  0  4 1 None
0x151 24 dc  0  32  0 r  y .   8  8  8  8 .  s  0  0  0  0  0  0  0  8 1 None
0x152 24 dc  0  32  0 r  y .   8  8  8  8 .  s  0  0  0  0  0  0  0 16 1 None
0x153 24 dc  0  32  0 r  y .   8  8  8  8 .  s  0 24  8  0  0  0  0  2 1 None
0x154 24 dc  0  32  0 r  y .   8  8  8  8 .  s  0 24  8  0  0  0  0  4 1 None
0x155 24 dc  0  32  0 r  y .   8  8  8  8 .  s  0 24  8  0  0  0  0  8 1 None
0x156 24 dc  0  32  0 r  y .   8  8  8  8 .  s  0 24  8  0  0  0  0 16 1 None
0x157 32 tc  0  32  0 r  y .   8  8  8  8 .  .  0  0  0  0  0  0  0  0 0 None
0x158 32 tc  0  32  0 r  . .   8  8  8  8 .  .  0  0  0  0  0  0  0  0 0 None
0x159 32 tc  0  32  0 r  y .   8  8  8  8 .  .  0 24  8  0  0  0  0  0 0 None
0x15a 32 tc  0  32  0 r  . .   8  8  8  8 .  .  0 24  8  0  0  0  0  0 0 None
0x15b 32 tc  0  32  0 r  y .   8  8  8  8 .  s  0  0  0  0  0  0  0  0 0 None
0x15c 32 tc  0  32  0 r  . .   8  8  8  8 .  s  0  0  0  0  0  0  0  0 0 None
0x15d 32 tc  0  32  0 r  y .   8  8  8  8 .  s  0 24  8  0  0  0  0  0 0 None
0x15e 32 tc  0  32  0 r  . .   8  8  8  8 .  s  0 24  8  0  0  0  0  0 0 None
0x15f 32 tc  0  32  0 r  y .   8  8  8  8 .  .  0 24  8  0  0  0  0  0 0 None
0x160 32 tc  0  32  0 r  y .   8  8  8  8 .  s  0 24  8  0  0  0  0  0 0 None

$ eglinfo 
EGL client extensions string:
    EGL_EXT_platform_base EGL_KHR_client_get_all_proc_addresses
    EGL_EXT_client_extensions EGL_KHR_debug EGL_EXT_platform_wayland
    EGL_EXT_platform_x11 EGL_MESA_platform_gbm
    EGL_MESA_platform_surfaceless

GBM platform:
i965_dri.so does not support the 0xffffffff PCI ID.
EGL API version: 1.4
EGL vendor string: Mesa Project
EGL version string: 1.4 (DRI2)
EGL client APIs: OpenGL OpenGL_ES 
EGL extensions string:
    EGL_EXT_buffer_age EGL_EXT_image_dma_buf_import EGL_KHR_cl_event2
    EGL_KHR_config_attribs EGL_KHR_create_context
    EGL_KHR_create_context_no_error EGL_KHR_fence_sync
    EGL_KHR_get_all_proc_addresses EGL_KHR_gl_colorspace
    EGL_KHR_gl_renderbuffer_image EGL_KHR_gl_texture_2D_image
    EGL_KHR_gl_texture_3D_image EGL_KHR_gl_texture_cubemap_image
    EGL_KHR_image EGL_KHR_image_base EGL_KHR_image_pixmap
    EGL_KHR_no_config_context EGL_KHR_reusable_sync
    EGL_KHR_surfaceless_context EGL_EXT_pixel_format_float
    EGL_KHR_wait_sync EGL_MESA_configless_context
    EGL_MESA_image_dma_buf_export
Configurations:
     bf lv colorbuffer dp st  ms    vis   cav bi  renderable  supported
  id sz  l  r  g  b  a th cl ns b    id   eat nd gl es es2 vg surfaces 
---------------------------------------------------------------------
0x01 32  0  8  8  8  8  0  0  0 0 0x34325241--         y  y  y     win
0x02 32  0  8  8  8  8 16  0  0 0 0x34325241--         y  y  y     win
0x03 32  0  8  8  8  8 24  0  0 0 0x34325241--         y  y  y     win
0x04 32  0  8  8  8  8 24  8  0 0 0x34325241--         y  y  y     win
0x05 32  0  8  8  8  8 32  0  0 0 0x34325241--         y  y  y     win
0x06 24  0  8  8  8  0  0  0  0 0 0x34325258--         y  y  y     win
0x07 24  0  8  8  8  0 16  0  0 0 0x34325258--         y  y  y     win
0x08 24  0  8  8  8  0 24  0  0 0 0x34325258--         y  y  y     win
0x09 24  0  8  8  8  0 24  8  0 0 0x34325258--         y  y  y     win
0x0a 24  0  8  8  8  0 32  0  0 0 0x34325258--         y  y  y     win
0x0b 16  0  5  6  5  0  0  0  0 0 0x36314752--         y  y  y     win
0x0c 16  0  5  6  5  0 16  0  0 0 0x36314752--         y  y  y     win
0x0d 16  0  5  6  5  0 24  0  0 0 0x36314752--         y  y  y     win
0x0e 16  0  5  6  5  0 24  8  0 0 0x36314752--         y  y  y     win
0x0f 16  0  5  6  5  0 32  0  0 0 0x36314752--         y  y  y     win

Wayland platform:
eglinfo: eglInitialize failed

X11 platform:
EGL API version: 1.4
EGL vendor string: Mesa Project
EGL version string: 1.4 (DRI2)
EGL client APIs: OpenGL OpenGL_ES 
EGL extensions string:
    EGL_ANDROID_blob_cache EGL_ANDROID_native_fence_sync
    EGL_CHROMIUM_sync_control EGL_EXT_buffer_age
    EGL_EXT_create_context_robustness EGL_EXT_image_dma_buf_import
    EGL_EXT_image_dma_buf_import_modifiers EGL_IMG_context_priority
    EGL_KHR_config_attribs EGL_KHR_create_context
    EGL_KHR_create_context_no_error EGL_KHR_fence_sync
    EGL_KHR_get_all_proc_addresses EGL_KHR_gl_colorspace
    EGL_KHR_gl_renderbuffer_image EGL_KHR_gl_texture_2D_image
    EGL_KHR_gl_texture_3D_image EGL_KHR_gl_texture_cubemap_image
    EGL_KHR_image EGL_KHR_image_base EGL_KHR_image_pixmap
    EGL_KHR_no_config_context EGL_KHR_reusable_sync
    EGL_KHR_surfaceless_context EGL_EXT_pixel_format_float
    EGL_KHR_wait_sync EGL_MESA_configless_context EGL_MESA_drm_image
    EGL_MESA_image_dma_buf_export EGL_NOK_texture_from_pixmap
    EGL_WL_bind_wayland_display
Configurations:
     bf lv colorbuffer dp st  ms    vis   cav bi  renderable  supported
  id sz  l  r  g  b  a th cl ns b    id   eat nd gl es es2 vg surfaces 
---------------------------------------------------------------------
0x01 32  0  8  8  8  8  0  0  0 0 0x41TC      a  y  y  y     win,pb,pix
0x02 32  0  8  8  8  8 24  8  0 0 0x41TC      a  y  y  y     win,pb,pix
0x03 24  0  8  8  8  0  0  0  0 0 0x41TC      y  y  y  y     win,pb,pix
0x04 24  0  8  8  8  0 24  8  0 0 0x41TC      y  y  y  y     win,pb,pix
0x05 32  0  8  8  8  8  0  0  2 1 0x41TC      a  y  y  y     win
0x06 32  0  8  8  8  8  0  0  4 1 0x41TC      a  y  y  y     win
0x07 32  0  8  8  8  8  0  0  8 1 0x41TC      a  y  y  y     win
0x08 32  0  8  8  8  8  0  0 16 1 0x41TC      a  y  y  y     win
0x09 32  0  8  8  8  8 24  8  2 1 0x41TC      a  y  y  y     win
0x0a 32  0  8  8  8  8 24  8  4 1 0x41TC      a  y  y  y     win
0x0b 32  0  8  8  8  8 24  8  8 1 0x41TC      a  y  y  y     win
0x0c 32  0  8  8  8  8 24  8 16 1 0x41TC      a  y  y  y     win
0x0d 24  0  8  8  8  0  0  0  2 1 0x41TC      y  y  y  y     win
0x0e 24  0  8  8  8  0  0  0  4 1 0x41TC      y  y  y  y     win
0x0f 24  0  8  8  8  0  0  0  8 1 0x41TC      y  y  y  y     win
0x10 24  0  8  8  8  0  0  0 16 1 0x41TC      y  y  y  y     win
0x11 24  0  8  8  8  0 24  8  2 1 0x41TC      y  y  y  y     win
0x12 24  0  8  8  8  0 24  8  4 1 0x41TC      y  y  y  y     win
0x13 24  0  8  8  8  0 24  8  8 1 0x41TC      y  y  y  y     win
0x14 24  0  8  8  8  0 24  8 16 1 0x41TC      y  y  y  y     win
0x15 32  0  8  8  8  8  0  0  0 0 0x42DC      a  y  y  y     win,pb,pix
0x16 32  0  8  8  8  8 24  8  0 0 0x42DC      a  y  y  y     win,pb,pix
0x17 24  0  8  8  8  0  0  0  0 0 0x42DC      y  y  y  y     win,pb,pix
0x18 24  0  8  8  8  0 24  8  0 0 0x42DC      y  y  y  y     win,pb,pix
0x19 32  0  8  8  8  8  0  0  2 1 0x42DC      a  y  y  y     win
0x1a 32  0  8  8  8  8  0  0  4 1 0x42DC      a  y  y  y     win
0x1b 32  0  8  8  8  8  0  0  8 1 0x42DC      a  y  y  y     win
0x1c 32  0  8  8  8  8  0  0 16 1 0x42DC      a  y  y  y     win
0x1d 32  0  8  8  8  8 24  8  2 1 0x42DC      a  y  y  y     win
0x1e 32  0  8  8  8  8 24  8  4 1 0x42DC      a  y  y  y     win
0x1f 32  0  8  8  8  8 24  8  8 1 0x42DC      a  y  y  y     win
0x20 32  0  8  8  8  8 24  8 16 1 0x42DC      a  y  y  y     win
0x21 24  0  8  8  8  0  0  0  2 1 0x42DC      y  y  y  y     win
0x22 24  0  8  8  8  0  0  0  4 1 0x42DC      y  y  y  y     win
0x23 24  0  8  8  8  0  0  0  8 1 0x42DC      y  y  y  y     win
0x24 24  0  8  8  8  0  0  0 16 1 0x42DC      y  y  y  y     win
0x25 24  0  8  8  8  0 24  8  2 1 0x42DC      y  y  y  y     win
0x26 24  0  8  8  8  0 24  8  4 1 0x42DC      y  y  y  y     win
0x27 24  0  8  8  8  0 24  8  8 1 0x42DC      y  y  y  y     win
0x28 24  0  8  8  8  0 24  8 16 1 0x42DC      y  y  y  y     win
```

</details>
