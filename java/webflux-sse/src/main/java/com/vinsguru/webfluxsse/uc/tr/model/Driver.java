package com.vinsguru.webfluxsse.uc.tr.model;

import com.fasterxml.jackson.annotation.JsonIgnoreProperties;
import com.fasterxml.jackson.annotation.JsonProperty;

import java.util.List;

@JsonIgnoreProperties(ignoreUnknown = true)
public class Driver {

    private List<Link> accessories;

    @JsonProperty("article_number")
    private Long articleNumber;

    private String name;

    @JsonProperty("group_id")
    private Long groupId;

    @JsonProperty("dimming_interface")
    private String dimmingInterface;

    // FIXME:error was there: it was 2x adjustable_current
    @JsonProperty("adjustable_output_current")
    private String adjustableOutputCurrent;

    @JsonProperty("form_factor")
    private String formFactor;

    @JsonProperty("emergency_operation")
    private Boolean emergencyOperation;

    @JsonProperty("adv_flex")
    private Boolean advancedFlex;

    @JsonProperty("channels")
    private Integer channels;

    @JsonProperty("current_max")
    private Double currentMax;

    @JsonProperty("current_min")
    private Double currentMin;

    @JsonProperty("ambient_temp_max")
    private Double tpMaxAmbient;

    @JsonProperty("ambient_temp_min")
    private Double tpMinAmbient;

    @JsonProperty("current_tolerance")
    private Double currentTolerance;

    @JsonProperty("current_ripple_tolerance")
    private Double currentRippleTolerance;

    private Double efficiency;

    private String layer;

    @JsonProperty("peak_tolerance")
    private Double peakTolerance;

    @JsonProperty("output_power")
    private Double outputPower;

    private String photo;

    @JsonProperty("selv")
    private String selv;

    @JsonProperty("type_of_mounting")
    private String typeOfMounting;

    @JsonProperty("mode_values")
    private List<ModeValues> modeValues;

    @JsonProperty("adjustable_current")
    private String adjustableCurrent;

    @JsonProperty("dimming_signal")
    private List<String> dimmingSignals;

    @JsonProperty("u_out")
    private Double uout;

    @JsonProperty("deviceGEN")
    private Boolean deviceGen;

    public List<Link> getAccessories() {
        return accessories;
    }

    public void setAccessories(List<Link> accessories) {
        this.accessories = accessories;
    }

    public Long getArticleNumber() {
        return articleNumber;
    }

    public void setArticleNumber(Long articleNumber) {
        this.articleNumber = articleNumber;
    }

    public String getName() {
        return name;
    }

    public void setName(String name) {
        this.name = name;
    }

    public Long getGroupId() {
        return groupId;
    }

    public void setGroupId(Long groupId) {
        this.groupId = groupId;
    }

    public Integer getChannels() {
        return channels;
    }

    public void setChannels(Integer channels) {
        this.channels = channels;
    }

    public Double getCurrentMax() {
        return currentMax;
    }

    public void setCurrentMax(Double currentMax) {
        this.currentMax = currentMax;
    }

    public Double getCurrentMin() {
        return currentMin;
    }

    public void setCurrentMin(Double currentMin) {
        this.currentMin = currentMin;
    }

    public Double getTpMaxAmbient() {
        return tpMaxAmbient;
    }

    public void setTpMaxAmbient(Double tpMaxAmbient) {
        this.tpMaxAmbient = tpMaxAmbient;
    }

    public Double getTpMinAmbient() {
        return tpMinAmbient;
    }

    public void setTpMinAmbient(Double tpMinAmbient) {
        this.tpMinAmbient = tpMinAmbient;
    }

    public Double getCurrentRippleTolerance() {
        return currentRippleTolerance;
    }

    public void setCurrentRippleTolerance(Double currentRippleTolerance) {
        this.currentRippleTolerance = currentRippleTolerance;
    }

    public Double getEfficiency() {
        return efficiency;
    }

    public void setEfficiency(Double efficiency) {
        this.efficiency = efficiency;
    }

    public String getLayer() {
        return layer;
    }

    public void setLayer(String layer) {
        this.layer = layer;
    }

    public Double getOutputPower() {
        return outputPower;
    }

    public void setOutputPower(Double outputPower) {
        this.outputPower = outputPower;
    }

    public String getPhoto() {
        return photo;
    }

    public void setPhoto(String photo) {
        this.photo = photo;
    }

    public String getSelv() {
        return selv;
    }

    public void setSelv(String selv) {
        this.selv = selv;
    }

    public String getTypeOfMounting() {
        return typeOfMounting;
    }

    public void setTypeOfMounting(String typeOfMounting) {
        this.typeOfMounting = typeOfMounting;
    }

    public List<ModeValues> getModeValues() {
        return modeValues;
    }

    public void setModeValues(List<ModeValues> modeValues) {
        this.modeValues = modeValues;
    }

    public Double getCurrentTolerance() {
        return currentTolerance;
    }

    public void setCurrentTolerance(Double currentTolerance) {
        this.currentTolerance = currentTolerance;
    }

    public String getDimmingInterface() {
        return dimmingInterface;
    }

    public void setDimmingInterface(String dimmingInterface) {
        this.dimmingInterface = dimmingInterface;
    }

    public String getAdjustableOutputCurrent() {
        return adjustableOutputCurrent;
    }

    public void setAdjustableOutputCurrent(String adjustableOutputCurrent) {
        this.adjustableOutputCurrent = adjustableOutputCurrent;
    }

    public String getFormFactor() {
        return formFactor;
    }

    public void setFormFactor(String formFactor) {
        this.formFactor = formFactor;
    }

    public Boolean getEmergencyOperation() {
        return emergencyOperation;
    }

    public void setEmergencyOperation(Boolean emergencyOperation) {
        this.emergencyOperation = emergencyOperation;
    }

    public Double getPeakTolerance() {
        return peakTolerance;
    }

    public void setPeakTolerance(Double peakTolerance) {
        this.peakTolerance = peakTolerance;
    }

    public String getAdjustableCurrent() {
        return adjustableCurrent;
    }

    public void setAdjustableCurrent(String adjustableCurrent) {
        this.adjustableCurrent = adjustableCurrent;
    }

    public Boolean getAdvancedFlex() {
        return advancedFlex;
    }

    public void setAdvancedFlex(Boolean advancedFlex) {
        this.advancedFlex = advancedFlex;
    }

    public List<String> getDimmingSignals() {
        return dimmingSignals;
    }

    public void setDimmingSignals(List<String> dimmingSignal) {
        this.dimmingSignals = dimmingSignal;
    }

    public Double getUout() {
        return uout;
    }

    public void setUout(Double uout) {
        this.uout = uout;
    }

    public Boolean getDeviceGen() {
        return deviceGen;
    }

    public void setDeviceGen(Boolean deviceGen) {
        this.deviceGen = deviceGen;
    }

    @Override
    public String toString() {
        return "Driver [accessories=" + accessories + ", articleNumber=" + articleNumber + ", name=" + name
            + ", groupId=" + groupId + ", dimmingInterface=" + dimmingInterface + ", adjustableOutputCurrent="
            + adjustableOutputCurrent + ", formFactor=" + formFactor + ", emergencyOperation=" + emergencyOperation
            + ", channels=" + channels + ", currentMax=" + currentMax + ", currentMin=" + currentMin
            + ", ambientTemperatureMin=" + tpMinAmbient + ", ambientTemperatureMax=" + tpMaxAmbient
            + ", currentTolerance=" + currentTolerance + ", currentRippleTolerance=" + currentRippleTolerance
            + ", efficiency=" + efficiency + ", layer=" + layer + ", peakTolerance=" + peakTolerance
            + ", outputPower=" + outputPower + ", photo=" + photo + ", selv=" + selv + ", typeOfMounting="
            + typeOfMounting + ", advancedFlex=" + advancedFlex + ", modeValues=" + modeValues
            + ", dimmingSignals=" + dimmingSignals + ", uout=" + uout + ", deviceGen=" + deviceGen + "]";
    }

}
