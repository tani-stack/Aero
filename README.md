# Aero OS v3 — Common OS for Drones, Cars और Robots

Aero अब एक सामान्य embedded OS के रूप में विकसित किया जा रहा है जो ड्रोन, autonomous कारें और रोबोट दोनों के लिए उपयोगी हो — एक ही कोर, साझा ड्राइवर मॉडल और यूनिफाइड vehicle‑एब्स्ट्रैक्शन के साथ। उद्देश्य: अलग‑अलग वाहन प्रकारों (Drone, Car, Robot) के लिए एक साझा, सुरक्षित और रीयल‑टाइम आधार देना ताकि कंट्रोल लॉजिक और सर्विसेज़ (EKF, PID, failsafe) को हार्डवेयर‑विशेषता से अलग रखा जा सके।

## मुख्य विशेषताएँ
- Secure Boot + TPM 2.0 measured boot (डिज़ाइन अनुरूप)
- O(1) RT Scheduler with 32 priority levels + preemption
- Buddy Frame Allocator + W^X paging
- Capability‑based IPC
- वास्तविक ड्राइवर सपोर्ट (IMU: ICM42688, Barometer: BMP390, GPS: Ublox, Lidar, Camera, DSHOT/ESCs और DC/Servo/Stepper प्रकार के मोटर)
- No‑alloc async executor (embedded friendly)
- EKF 16‑state + PID controllers (services में उपयोग के लिए)
- Failsafe + Watchdog

## क्या नया/ज़रूरी जानकारी
- यह प्रोजेक्ट अब "common OS" के रूप में है: drivers/ और vehicle/ मॉड्यूलों में abstractions इसलिए डिज़ाइन किए गए हैं ताकि एक ही कोर का उपयोग ड्रोन, कार और रोबोट पर किया जा सके।
- हार्डवेयर‑स्पेसिफिक HAL और बस‑इम्प्लीमेंटेशन (SPI, I2C, UART, PWM, GPIO, timers) हर बोर्ड पर अलग होंगे — platform/ और drivers/src/hal/ में बोर्ड‑लिंक करने के लिए जगह मौजूद है।
- कुछ ड्राइवर/रीड फ़ंक्शन्स अभी high‑level या example‑स्तर के हैं (स्टब/सिम्युलेटेड डेटा) — रीयल हार्डवेर पर चलाने से पहले HAL और bus glue तथा सेंसर‑काँफिग को पूरा और सत्यापित करना ज़रूरी है।

## Build और QEMU पर टेस्ट
QEMU पर त्वरित रन के लिए (dev/test):

```sh
cargo build --release
qemu-system-aarch64 -M virt -cpu cortex-a72 -kernel target/aarch64-unknown-none/release/aero-kernel
```

नोट: असली बोर्ड पर डिप्लॉय करने के लिए target, लिंक‑स्क्रिप्ट, और बोर्ड‑विशेष HAL/bootloader की आवश्यक्ता होगी।

## प्रोजेक्ट संरचना (संक्षेप)
```
boot/        # bootloader और प्रारंभिक init
kernel/      # scheduler, memory, paging, core OS
drivers/      # sensor और actuator drivers, bus और hal abstractions
libs/         # साझा प्रकार और यूटिलिटी (e.g., aero_types)
runtime/      # async executor, runtime helpers
services/     # EKF, PID, failsafe, higher-level control logic
```

## अगला कदम (आप कर सकते हैं)
- अगर आप चाहते हैं कि मैं README में और जानकारी जोड़ दूँ (जैसे contribution guide, board support list, required hardware), बताइए कौन‑सा बोर्ड/सेंसर आप प्राथमिकता देना चाहते हैं।
- मैं platform/<board> के लिए एक example HAL टेम्पलेट भी बना सकता हूँ जो आपके बोर्ड के पिन‑मैप और बस‑डिवाइस को दिखाए।

---

Aero — common OS vision: एक ही कोर, बहुत से वाहनों के लिए।
