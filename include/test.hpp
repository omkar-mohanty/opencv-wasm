#include <string>
#include <memory>
#include <opencv4/opencv2/imgcodecs.hpp>
namespace manual {
	std::unique_ptr<std::string> test() {
	return std::make_unique<std::string>(std::string("Hello"));
}}
