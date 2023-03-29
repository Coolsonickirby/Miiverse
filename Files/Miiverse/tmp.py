import numpy as np
import cv2

im = cv2.imread('imgs/0.png')
row, col = im.shape[:2]
bottom = im[row-2:row, 0:col]

bordersize = 5
border = cv2.copyMakeBorder(
    im,
    top=bordersize,
    bottom=bordersize,
    left=bordersize,
    right=bordersize,
    borderType=cv2.BORDER_CONSTANT,
    value=[0, 0, 255]
)

cv2.imshow('border', border)
cv2.waitKey(0)
cv2.destroyAllWindows()