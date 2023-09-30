#include <atomic>
#include <chrono>
#include <cstdint>
#include <exception>
#include <iostream>
#include <memory>
#include <thread>
#include <utility>

template <typename T>
class Sender;

template <typename T>
class Receiver;

template <typename T>
class Channel {
    std::unique_ptr<T> message { nullptr };
    std::atomic_bool ready { false };
    friend class Sender<T>;
    friend class Receiver<T>;
};

template <typename T>
auto make_channel();

template <typename T>
class Sender {
public:
    static auto send(std::unique_ptr<Sender<T>> sender, std::unique_ptr<T> message) -> void
    {
        sender->channel->message = std::move(message);
        sender->channel->ready.store(true, std::memory_order_release);
    }

private:
    std::shared_ptr<Channel<T>> channel;
    friend auto make_channel<T>();

    static auto create(std::shared_ptr<Channel<T>> c) -> std::unique_ptr<Sender<T>>
    {
        return std::unique_ptr<Sender<T>> { new Sender<T>(std::move(c)) };
    }

    explicit Sender(std::shared_ptr<Channel<T>> c)
        : channel { std::move(c) }
    {
    }
};

template <typename T>
class Receiver {
public:
    auto is_ready() -> bool
    {
        return channel->ready.load(std::memory_order_relaxed);
    }

    static auto receive(std::unique_ptr<Receiver<T>> receiver) -> std::unique_ptr<T>
    {
        if (!receiver->channel->ready.load(std::memory_order_acquire)) {
            std::terminate();
        }
        return std::move(receiver->channel->message);
    }

private:
    std::shared_ptr<Channel<T>> channel;
    friend auto make_channel<T>();

    explicit Receiver(std::shared_ptr<Channel<T>> c)
        : channel { std::move(c) }
    {
    }

    static auto create(std::shared_ptr<Channel<T>> c) -> std::unique_ptr<Receiver<T>>
    {
        return std::unique_ptr<Receiver<T>> { new Receiver<T>(std::move(c)) };
    }
};

template <typename T>
auto make_channel()
{
    auto channel = std::make_shared<Channel<T>>();
    auto channel_copy = channel;
    auto sender = Sender<T>::create(std::move(channel));
    auto receiver = Receiver<T>::create(std::move(channel_copy));
    return std::make_pair(std::move(sender), std::move(receiver));
}

int main()
{
    auto [sender, receiver] = make_channel<std::int32_t>();

    auto t1 = std::thread([&sender] {
        // Prepare the data
        std::this_thread::sleep_for(std::chrono::milliseconds { 1000 });

        // Send the data
        Sender<std::int32_t>::send(std::move(sender), std::make_unique<std::int32_t>(42));
    });

    auto t2 = std::thread([&receiver] {
        while (!receiver->is_ready()) {
            std::this_thread::yield();
        }

        std::cout << "The answer: " << *Receiver<std::int32_t>::receive(std::move(receiver)) << std::endl;
    });

    t1.join();
    t2.join();
    return 0;
}
