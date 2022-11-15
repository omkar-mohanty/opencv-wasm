#pragma once
#include <string>
#include <vector>
#include <memory>
#include <opencv4/opencv2/imgcodecs.hpp>
#include <opencv4/opencv2/imgproc.hpp>
#include <opencv4/opencv2/highgui.hpp>
#include <opencv4/opencv2/core.hpp>
#include <opencv4/opencv2/core/mat.hpp>
#include <opencv4/opencv2/core/types.hpp>
namespace manual {
	typedef cv::Mat Mat;
	typedef cv::Point Point;
	typedef cv::Size Size;  	
	std::unique_ptr<Mat> imread(const std::string& filename, int32_t flags) {
		return std::make_unique<Mat>(cv::imread(filename,flags));
	}

	bool imwrite(const std::string& filename, std::unique_ptr<Mat> img) {
		return cv::imwrite(filename,*img.get());
	}

	std::unique_ptr<Mat> imdecode(std::unique_ptr<Mat> buf, int32_t flags) {
		return std::make_unique<Mat>(cv::imdecode(*buf.get(), flags));
	}
	
	
	void bilateralFilter(std::unique_ptr<Mat> src, std::unique_ptr<Mat> dst, int32_t d, double sigmaColor, double sigmaSpace, int32_t borderType) {
		cv::bilateralFilter(*src.get(), *dst.get(), d,sigmaColor, sigmaSpace, borderType);
	}

	void blur(std::unique_ptr<Mat> src,  std::unique_ptr<Mat> dst, std::unique_ptr<Size> ksize, std::unique_ptr<Point> anchor, int32_t borderType) {
		cv::blur( *src.get(), *dst.get(), *ksize.get(),*anchor.get(),borderType);
	}

	void boxFilter(std::unique_ptr<Mat> src,  std::unique_ptr<Mat> dst, int32_t ddepth, std::unique_ptr<Size> ksize, std::unique_ptr<Point> anchor, bool normalize, int32_t borderType) {
	cv::boxFilter(*src.get(),*dst.get(),  ddepth, *ksize.get(),*anchor.get(),normalize,borderType);	
	}

	void namedWindow(const std::string& winname, int32_t flags ) {
		cv::namedWindow(winname,flags);
	}

	void imshow(const std::string& winname, std::unique_ptr<Mat> mat) {
		cv::imshow(winname, *mat.get());
	}

	std::unique_ptr<Mat> mat() {
		return std::make_unique<Mat>();
	}

	std::unique_ptr<Size> size(int32_t i, int32_t j) {
		return std::make_unique<Size>(i, j);
	}

	std::unique_ptr<Point> point(int32_t i, int32_t j) {
		return std::make_unique<Point>(i, j);
	}
}

