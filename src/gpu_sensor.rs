use nvml_wrapper::Nvml;

use nvml_wrapper::enum_wrappers::device::TemperatureSensor;
use nvml_wrapper::error::NvmlError;
use nvml_wrapper::struct_wrappers::device::{MemoryInfo, Utilization};

use crate::data::{SensorDataValues, SensorValues};

struct GPUstat {
    utilization_rates: Result<Utilization, NvmlError>,
    memory_info: Result<MemoryInfo, NvmlError>,
    temperature: Result<u32, NvmlError>,
}

fn read_gpu_stat(device: &nvml_wrapper::Device) -> GPUstat {
    let gpustat = GPUstat {
        utilization_rates: device.utilization_rates(),
        memory_info: device.memory_info(),
        temperature: device.temperature(TemperatureSensor::Gpu),
    };

    return gpustat;
}

pub fn sensor_values() -> Result<SensorDataValues, nvml_wrapper::error::NvmlErrorWithSource> {
    let nvml = Nvml::init()?;
    let device = nvml.device_by_index(0)?;
    let result = generate_json(device);
    Ok(result)
}

fn generate_json(device: nvml_wrapper::Device) -> SensorDataValues {
    let gpustat = read_gpu_stat(&device);
    let mut result = SensorDataValues { values: Vec::new() };

    let utilization_rates = match gpustat.utilization_rates {
        Ok(utilization_rates) => format!("{}", utilization_rates.gpu),
        Err(_err) => "".to_string(),
    };
    let sensor_data = SensorValues {
        value_type: "gpu_load".to_string(),
        value: utilization_rates,
    };
    result.values.push(sensor_data);

    let memory_info = match gpustat.memory_info {
        Ok(memory_info) => format!(
            "{:.6}",
            (memory_info.used as f64 / memory_info.total as f64).to_string()
        ),
        Err(_err) => "".to_string(),
    };
    let sensor_data = SensorValues {
        value_type: "gpu_memory".to_string(),
        value: memory_info,
    };
    result.values.push(sensor_data);

    let temperature = match gpustat.temperature {
        Ok(temperature) => format!("{}", temperature),
        Err(_err) => "".to_string(),
    };
    let sensor_data = SensorValues {
        value_type: "gpu_temperature".to_string(),
        value: temperature,
    };
    result.values.push(sensor_data);

    return result;
}
