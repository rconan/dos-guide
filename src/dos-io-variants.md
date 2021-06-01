# IO variants

```rust,ignore
    // Uniform Wind Pressure
    OSSTopEnd6F,  // Top-End
    MCM2Lcl6F,    // M2 segments
    OSSTruss6F,   //Truss
    OSSM1Lcl6F,   // M1 segments
    OSSCellLcl6F, // M1 segment cells
    OSSGIR6F,     // GIR
    OSSCRING6F,   // C-rings
    // Axial wind forces on M1 mirror segments
    M1DistributedWindf,
    // Axial displacement of M1 segment surface nodes
    M1Segment1AxialD,
    M1Segment2AxialD,
    M1Segment3AxialD,
    M1Segment4AxialD,
    M1Segment5AxialD,
    M1Segment6AxialD,
    M1Segment7AxialD,
    // M1 hardpoints
    OSSHarpointDeltaF, // forces
    OSSHardpointD,     // displacements
    // M1 Actuators forces applied to back-side of M1 segments
    M1ActuatorsSegment1,
    M1ActuatorsSegment2,
    M1ActuatorsSegment3,
    M1ActuatorsSegment4,
    M1actuatorsSegment5,
    M1actuatorsSegment6,
    M1ActuatorsSegment7,
    // M1 fans
    OSSM1FansLcl6F,
    OSSM1FansLcl6D,
    // Payloads
    OSSPayloads6F,
    OSSPayloads6D,
    // Mount Drives
    OSSAzDriveF,  // azimuth drive
    OSSElDriveF,  // elevation drive
    OSSGIRDriveF, // GIR drive
    OSSAzDriveD,
    OSSElDriveD,
    OSSGIRDriveD,
    // Mount Drives
    OSSAzDriveTorque,
    OSSElDriveTorque,
    OSSRotDriveTorque,
    OSSAzEncoderAngle,
    OSSElEncoderAngle,
    OSSRotEncoderAngle,
    // Azimuth, elevation, rotation drive torques
    SlewTorques,
    // Line of sight
    OSSM1LOS,  // M1
    MCM2LOS6D, // M2
    // Base of Pier
    OSSBASE6F,
    // Inertial Measurement Units
    OSSIMUs6d,
    // M2 Positioners
    MCM2SmHexF,
    // ASM Proof Mass Actuators
    MCM2PMA1F,
    // ASM
    MCM2CP6F,  // Cold plates
    MCM2RB6F,  // Reference bodies
    MCM2CP6D,  // Cold plates
    MCM2RB6D,  // Reference bodies
    MCM2Lcl6D, // Face sheets
    MCASMCOG6F,
    MCASMCOG6D,
    //
    MCM2TE6F,
    MCM2TEIF6F,
    OSSTrussTEIF6f,
    MCM2GravCS0,
    MCM2PZTS1F,
    MCM2PZTS2F,
    MCM2PZTS3F,
    MCM2PZTS4F,
    MCM2PZTS5F,
    MCM2PZTS6F,
    MCM2PZTS7F,
    MCM2SmallS16F,
    MCM2SmallS26F,
    MCM2SmallS36F,
    MCM2SmallS46F,
    MCM2SmallS56F,
    MCM2SmallS66F,
    MCM2SmallS76F,
    OSSGravCS0,
    OSSTrussIF6D,
    OSSGIR6D,
    OSSCRING6D,
    OSSBASE6D,
    OSSM1Lcl,
    OSSTruss6d,
    OSSCellLcl,
    MCM2SmallS16D,
    MCM2PZTS1D,
    MCM2SmallS26D,
    MCM2PZTS2D,
    MCM2SmallS36D,
    MCM2PZTS3D,
    MCM2SmallS46D,
    MCM2PZTS4D,
    MCM2SmallS56D,
    MCM2PZTS5D,
    MCM2SmallS66D,
    MCM2PZTS6D,
    MCM2SmallS76D,
    MCM2PZTS7D,
    M1SurfacesD,
    M1EdgeSensors,
    MCM2CP1D,
    MCM2SmHexD,
    M2edgesensors,
    MCM2TEIF6D,
    MCM2TE6D,
    M2ReferenceBody1AxialD,
    M2ReferenceBody2AxialD,
    M2ReferenceBody3AxialD,
    M2ReferenceBody4AxialD,
    M2ReferenceBody5AxialD,
    M2ReferenceBody6AxialD,
    M2ReferenceBody7AxialD,
    MountCmd,
    // M1 control
    M1HPCmd,
    M1HPLC,
    M1CGFM,
    SensorData,
    SrcWfeRms,
    Pssn
```
