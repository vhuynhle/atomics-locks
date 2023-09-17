add_executable(01_relaxed_ordering_no_sync src/01_relaxed_ordering_no_sync.cpp)
add_executable(01_relaxed_ordering_no_circular_computation
               src/01_relaxed_ordering_no_circular_computation.cpp)
add_executable(relaxed_ordering_stop_flag src/relaxed_ordering_stop_flag.cpp)
add_executable(relaxed_ordering_progress_reporting
               src/relaxed_ordering_progress_reporting.cpp)
add_executable(relaxed_ordering_progress_reporting_multiworkers
  src/relaxed_ordering_progress_reporting_multiworkers.cpp)
add_executable(compare_exchange_id_allocation src/compare_exchange_id_allocation.cpp)
add_executable(compare_exchange_lazy_init src/compare_exchange_lazy_init.cpp)
