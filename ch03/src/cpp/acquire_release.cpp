#include <atomic>
#include <chrono>
#include <cstdint>
#include <iostream>
#include <thread>

namespace {
auto sData = std::uint64_t { 0U };
auto sReady = std::atomic_bool { false };
}

int main()
{
    auto t = std::thread { []() {
        // Modifying sData is safe here because we haven't set the sReady flag
        // => Nothing else is accessing data
        sData = 123;
        sReady.store(true, std::memory_order_release); // Everything before this stores ..
    } };

    while (!sReady.load(std::memory_order_acquire)) { // .. is visible after this loads `true`
        std::this_thread::sleep_for(std::chrono::milliseconds { 100 });
        std::cout << "Waiting ...\n";
    }

    // Accessing sData here is safe because READY is already set,
    // meaning that nothing is mutating DATA
    std::cout << sData << std::endl;

    t.join();

    return 0;
}
