#pragma once
#include <string>
#include <vector>
#include <memory>
#include <opencv4/opencv2/imgproc.hpp>
#include <opencv4/opencv2/core.hpp>
#include <opencv4/opencv2/core/mat.hpp>
namespace manual {
	typedef cv::Mat Mat;
	typedef cv::_InputArray InputArray;
	typedef cv::_OutputArray OutputArray;
	void bilateralFilter(std::unique_ptr<InputArray> src, std::unique_ptr<OutputArray> dst, int32_t d, double sigmaColor, double sigmaSpace, int32_t borderType) {
		cv::bilateralFilter(*src.get(), *dst.get(), d,sigmaColor, sigmaSpace);
	}

	void blur() {
	}
}

