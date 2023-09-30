#include <chrono>
#include <condition_variable>
#include <cstdint>
#include <deque>
#include <iostream>
#include <mutex>
#include <thread>
#include <vector>

template <typename T>
class Channel {
public:
    Channel() = default;

    void send(T message)
    {
        {
            std::lock_guard guard { mutex };
            queue.push_back(message);
        }
        cond.notify_one();
    }

    T receive()
    {
        std::unique_lock lock { mutex };
        cond.wait(lock, [this]() { return !queue.empty(); });
        const auto message = queue.front();
        queue.pop_front();
        return message;
    }

private:
    std::deque<T> queue;
    std::mutex mutex;
    std::condition_variable cond;
};

int main()
{
    auto c = Channel<std::int32_t> {};
    std::vector<std::jthread> handles;
    for (std::int32_t i { 0 }; i < 10; ++i) {
        handles.emplace_back([&c, i]() {
            const auto msg = c.receive();
            std::cout << "Thread " << i << " receives " << msg << std::endl;
        });
    }

    auto producer = std::jthread([&c]() {
        for (std::int32_t i { 0 }; i < 10; ++i) {
            c.send(i * i);
            std::this_thread::sleep_for(std::chrono::milliseconds { 100 });
        }
    });

    return 0;
}
