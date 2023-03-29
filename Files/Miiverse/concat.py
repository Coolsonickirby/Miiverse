import cv2

imgs = []
srcDir = "imgBordered"
i = 0
for x in range(0, 5):
    grp = []
    for y in range(0, 5):
        grp.append(cv2.imread(f"{srcDir}/{i}.png"))
        i += 1
    imgs.append(cv2.hconcat(grp))

cv2.imwrite('bigPost.png', cv2.vconcat(imgs))