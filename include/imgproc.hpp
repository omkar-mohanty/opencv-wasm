#pragma once
#include <string>
#include <vector>
#include <memory>
#include <opencv4/opencv2/imgproc.hpp>
#include <opencv4/opencv2/core.hpp>
#include <opencv4/opencv2/core/mat.hpp>
#include <opencv4/opencv2/core/types.hpp>
namespace manual {
	typedef cv::Mat Mat;
	typedef cv::_InputArray InputArray;
	typedef cv::_OutputArray OutputArray;
	typedef cv::Point Point;
	typedef cv::Size Size;
	void bilateralFilter(std::unique_ptr<InputArray> src, std::unique_ptr<OutputArray> dst, int32_t d, double sigmaColor, double sigmaSpace, int32_t borderType) {
		cv::bilateralFilter(*src.get(), *dst.get(), d,sigmaColor, sigmaSpace, borderType);
	}

	void blur(std::unique_ptr<InputArray> src,  std::unique_ptr<OutputArray> dst, std::unique_ptr<Size> ksize, std::unique_ptr<Point> anchor, int32_t borderType) {
		cv::blur( *src.get(), *dst.get(), *ksize.get(),*anchor.get(),borderType);
	}
}

