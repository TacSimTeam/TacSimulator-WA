#include <iostream>
#include <boost/asio.hpp>
#include <chrono>

int main() {
    std::string serialPortPath = "/dev/tty.usbserial-AI05V9GR";

    try {
        boost::asio::io_service io_service;

        boost::asio::serial_port serialPort(io_service, serialPortPath);

        auto start = std::chrono::high_resolution_clock::now();
        while (true) {
            char c;
            boost::asio::read(serialPort, boost::asio::buffer(&c, 1));

            std::cout << c;
            if (c == ';') {
              break;
            }
        }
        auto end = std::chrono::high_resolution_clock::now();

        auto elapsedTime = std::chrono::duration_cast<std::chrono::milliseconds>(end - start);
        std::cout << "Elapsed time: " << elapsedTime.count() << " milliseconds" << std::endl;
    } catch (std::exception& e) {
        std::cerr << "Exception: " << e.what() << std::endl;
        return 1;
    }

    return 0;
}

