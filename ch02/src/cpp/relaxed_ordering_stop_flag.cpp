#include <atomic>
#include <iostream>
#include <string>
#include <thread>

using namespace std::chrono_literals;

void some_work() noexcept
{
    std::this_thread::sleep_for(1s);
}

int main()
{
    auto stop = std::atomic_bool { false };

    auto background_thread = std::jthread {
        [&]() {
            while (!stop.load(std::memory_order_relaxed)) {
                some_work();
            }
        }
    };

    std::string line {};
    while (true) {
        std::getline(std::cin, line);
        if (line == "help") {
            std::cout << "Commands: help, stop\n";
        } else if (line == "stop") {
            stop.store(true, std::memory_order_relaxed);
            break;
        } else {
            std::cout << "Unknown command: " << line << '\n';
        }
    }

    return 0;
}
