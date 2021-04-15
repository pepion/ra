package com.vinsguru.webfluxsse.uc.tr.model;

import com.fasterxml.jackson.annotation.JsonProperty;

public class ModeValues {
    @JsonProperty("max_forward_voltage")
    private Double maxForwardVoltage;
    @JsonProperty("min_forward_voltage")
    private Double minForwardVolatege;
    @JsonProperty("max_output_power")
    private Double maxOutputPower;
    @JsonProperty("output_current")
    private Double outputCurrent;

    public Double getMaxForwardVoltage() {
        return maxForwardVoltage;
    }

    public void setMaxForwardVoltage(Double maxForwardVoltage) {
        this.maxForwardVoltage = maxForwardVoltage;
    }

    public Double getMinForwardVolatege() {
        return minForwardVolatege;
    }

    public void setMinForwardVolatege(Double minForwardVolatege) {
        this.minForwardVolatege = minForwardVolatege;
    }

    public Double getMaxOutputPower() {
        return maxOutputPower;
    }

    public void setMaxOutputPower(Double maxOutputPower) {
        this.maxOutputPower = maxOutputPower;
    }

    public Double getOutputCurrent() {
        return outputCurrent;
    }

    public void setOutputCurrent(Double outputCurrent) {
        this.outputCurrent = outputCurrent;
    }

    @Override
    public String toString() {
        return "ModeValues [maxForwardVoltage=" + maxForwardVoltage + ", minForwardVolatege=" + minForwardVolatege
                + ", maxOutputPower=" + maxOutputPower + ", outputCurrent=" + outputCurrent + "]";
    }
}
