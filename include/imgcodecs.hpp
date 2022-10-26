#pragma once
#include <string>
#include <vector>
#include <memory>
#include <opencv4/opencv2/imgcodecs.hpp>
#include <opencv4/opencv2/core.hpp>
#include <opencv4/opencv2/core/mat.hpp>
namespace manual {
	typedef cv::Mat Mat;
	typedef cv::_InputArray InputArray;
	std::unique_ptr<Mat> imread(const std::string& filename, int32_t flags) {
		return std::make_unique<Mat>(cv::imread(filename,flags));
	}

	bool imwrite(const std::string& filename, std::unique_ptr<InputArray> img, const std::vector<int32_t> &params ) {
		return cv::imwrite(filename,*img.get(),params);
	}

	std::unique_ptr<Mat> imdecode(std::unique_ptr<InputArray> buf, int32_t flags) {
		return std::make_unique<Mat>(cv::imdecode(*buf.get(), flags));
	}
}

