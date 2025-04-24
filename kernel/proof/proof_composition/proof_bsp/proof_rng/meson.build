# SPDX-FileCopyrightText: 2023 Ledger SAS
# SPDX-FileCopyrightText: 2025 H2Lab
# SPDX-License-Identifier: Apache-2.0

# specify an ELF including framaC main. This forges the correct flags in the
# compile_commands.json so that framaC can use it. Although it should never be
# built by meson, but instead json metadata being used by framaC to forge the
# correct cpp args
framac_bsp_rng_stub_entrypoint = files(
  'main.c',
)

framac_bsp_rng_stub_elf = executable(
  'framac_bsp_rng_stub_exe',
  name_suffix: 'elf',
  sources: [
    framac_bsp_rng_stub_entrypoint,
    framac_dep,
  ],
  include_directories: kernel_inc,
  dependencies: [ external_deps, proof_lib_dep ],
  c_args: [ target_arch_args, global_build_args ],
  install: false,
  build_by_default: false,
)

test(
  'frama-C-wp-bsp-rng',
  frama_c,
  args: [
    framac_bsp_rng_stub_entrypoint,
    meson.project_source_root()+'/kernel/src/drivers/rng/stm32f4-rng.c',
    framac_gen_opts,
    '-absolute-valid-range', '0x40000000-0xE000F000',
    framac_rte_eva_opts,
    '-eva-log', 'a:'+join_paths(meson.current_build_dir(), 'sentry-rte-eva.log'),
    '-eva-report-red-statuses', join_paths(meson.current_build_dir(), 'sentry-red-statuses.log'),
    '-metrics',
    '-metrics-eva-cover',meson.project_source_root()+'/kernel/src/drivers/rng/stm32f4-rng.c',
    '-then', framac_wp_opts,
    '-wp-fct','rng_probe,rng_get',
    '-wp-log', 'a:'+join_paths(meson.current_build_dir(), 'sentry-wp.log'),
    '-save',join_paths(meson.current_build_dir(), 'sentry-bsp-rng.wp'),
    '-then',
    framac_md_opts,
  ],
  env: {'FRAMAC_SESSIfON': join_paths(meson.current_build_dir(), '.frama-c')},
  depends: [ framac_dep ],
  suite: 'proof',
  timeout: 1200,
)
