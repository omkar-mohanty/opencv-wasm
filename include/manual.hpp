#pragma once
#include <string>
#include <memory>
#include <opencv4/opencv2/imgcodecs.hpp>
namespace manual {
	std::unique_ptr<cv::Mat> imread(std::string& filename, int flags) {
		return std::make_unique<cv::Mat>(cv::imread(filename,flags));
	}
}

