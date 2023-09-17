#include <atomic>
#include <cstddef>
#include <cstdint>
#include <iostream>
#include <limits>
#include <stdexcept>
#include <syncstream>
#include <thread>
#include <vector>

std::uint64_t allocate_new_id()
{
    static const auto kLimits { std::numeric_limits<std::uint64_t>::max() };

    static auto next_id = std::atomic_uint64_t { 0 };
    auto id = next_id.load(std::memory_order_relaxed);

    while (true) {
        // Check the limit before modifying next_id
        // to ensure that there is no overflow
        if (id == kLimits) {
            throw std::runtime_error { "Too many IDs!" };
        }

        if (next_id.compare_exchange_weak(id, id + 1, std::memory_order_relaxed)) {
            // OK, the value has been successfully changed
            return id + 1;
        } else {
            // The value has been changed by another thread, but not this one.
            // Try again in the next iteration.
        }
    }

    return id;
}

int main()
{
    static constexpr std::size_t kNumThreads { 20 };
    std::vector<std::jthread> handles;
    handles.reserve(kNumThreads);

    for (std::size_t i { 0 }; i < kNumThreads; ++i) {
        handles.emplace_back([]() {
            const auto id { allocate_new_id() };
            std::osyncstream { std::cout } << "Thread "
                                           << std::this_thread::get_id()
                                           << " -> "
                                           << id << '\n';
        });
    }

    return 0;
}
