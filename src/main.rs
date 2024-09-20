// Трейт для HDMI интерфейса
trait Hdmi {
    fn send_hdmi_signal(&self) -> String;
}

// Структура для устройства, использующего HDMI
struct HDMIOutput;

impl Hdmi for HDMIOutput {
    fn send_hdmi_signal(&self) -> String {
        String::from("Сигнал HDMI")
    }
}

// Трейт для VGA интерфейса
trait Vga {
    fn send_vga_signal(&self) -> String;
}

//Структура для устройства, использующего VGA
struct VgaInput;

impl Vga for VgaInput {
    fn send_vga_signal(&self) -> String {
        String::from("Сигнал VGA")
    }
}

// Адаптер для подключения HDMI устройства к VGA
struct HdmiToVgaAdapter {
    hdmi_output: HDMIOutput,
}

impl Vga for HdmiToVgaAdapter {
    fn send_vga_signal(&self) -> String {
        // Преобразуем HDMI сигнал в VGA сигнал
        let hdmi_signal = self.hdmi_output.send_hdmi_signal();
        String::from("Преобразованный ") + &hdmi_signal
    }
}

fn main() {
    let hdmi_output = HDMIOutput;
    let adapter = HdmiToVgaAdapter { hdmi_output };

    // Используем адаптер, чтобы вызвать метод VGA интерфейса
    println!("{}", adapter.send_vga_signal()); // Вывод: "Преобразованный Сигнал HDMI"
}
