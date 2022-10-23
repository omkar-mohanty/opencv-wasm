#pragma once
#include <string>
#include <vector>
#include <memory>
#include <opencv4/opencv2/imgcodecs.hpp>
#include <opencv4/opencv2/core.hpp>
namespace manual {
	typedef cv::Mat Mat;
	std::unique_ptr<Mat> imread(const std::string& filename, int32_t flags) {
		return std::make_unique<Mat>(cv::imread(filename,flags));
	}
}

