<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>HCA_Dynamiccomments</name>
   <tag></tag>
   <elementGuidId>c0894969-f9c0-49be-b8ae-f45ad2d1129e</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;\u003cns7:Envelope xmlns:ns2\u003d\&quot;http://www.travelport.com/otm/builtins/tp/msgcommon/v1_0_0\&quot; xmlns:ns3\u003d\&quot;http://www.travelport.com/sandbox/hca/schemas/hca/v1\&quot; xmlns:ns4\u003d\&quot;http://www.opentravel.org/OTM/Common/v0\&quot; xmlns:ns5\u003d\&quot;http://www.travelport.com/schemas/hca/v0\&quot; xmlns:ns6\u003d\&quot;http://www.travelport.com/otm/builtins/tp/message/v1_0_0\&quot; xmlns:ns7\u003d\&quot;http://schemas.xmlsoap.org/soap/envelope/\&quot;\u003e\n    \u003cns7:Header\u003e\n        \u003cns5:SessionContextHCA\u003e\n            \u003cns5:E2ETrackingID\u003ebdf6e1fe-efcf-4c71-9690-d2ecd963aa9f\u003c/ns5:E2ETrackingID\u003e\n            \u003cns5:ChildTrackingID\u003e12528673-806a-4aba-afb2-e0e3f6ea89b6\u003c/ns5:ChildTrackingID\u003e\n            \u003cns5:OriginApplication\u003eUAPI\u003c/ns5:OriginApplication\u003e\n            \u003cns5:CustomerProfileID\u003e${sCustomerProfileId}\u003c/ns5:CustomerProfileID\u003e\n            \u003cns5:SessionDisposition\u003eMIDDLE\u003c/ns5:SessionDisposition\u003e\n            \u003cns5:ExternalSessionToken\u003e${ExtSessionToken}\u003c/ns5:ExternalSessionToken\u003e\n            \u003cns5:PseudoCityCode\u003e${sPCC}\u003c/ns5:PseudoCityCode\u003e\n            \u003c!--\u003cns5:TPFSYSID\u003eE735615\u003c/ns5:TPFSYSID\u003e--\u003e\n        \u003c/ns5:SessionContextHCA\u003e\n    \u003c/ns7:Header\u003e\n    \u003cns7:Body\u003e\n        \u003cns3:CommandRQ ReturnSoapFault\u003d\&quot;false\&quot;\u003e\n            \u003cns3:HCARequest retryCnt\u003d\&quot;0\&quot;\u003e\n                \u003cns3:SysMgmt_RQ\u003e\n                    \u003cns3:HCASysMgmtSessProp_SysMgmtCmd mep\u003d\&quot;RR\&quot;\u003e\n                        \u003cns3:Endpoint\u003e1G\u003c/ns3:Endpoint\u003e\n                        \u003c!--\u003cns3:GTIDLEIDOvrd\u003eFE425C\u003c/ns3:GTIDLEIDOvrd\u003e--\u003e\n                        \u003cns3:DiagCmdCds timeStamp\u003d\&quot;EntryAndExit\&quot;\u003e\n                            \u003cns3:SysRsrcsCnsmd\u003eNodes\u003dTPF; ecbExistTime; CPUMills; DiskRd; DiskWrt; PrgCalls\u003c/ns3:SysRsrcsCnsmd\u003e\n                            \u003cns3:SysOpsStats\u003eNodes\u003dTPF;IStrm\u003dCurrent\u003c/ns3:SysOpsStats\u003e\n                        \u003c/ns3:DiagCmdCds\u003e\n                        \u003cns3:Elog\u003eNone\u003c/ns3:Elog\u003e\n                    \u003c/ns3:HCASysMgmtSessProp_SysMgmtCmd\u003e\n                    \u003cns3:DiagnosticCmds\u003e\n                        \u003cns3:Item nm\u003d\&quot;01\&quot; val\u003d\&quot;when\u003d2017-12-20T15:23:58.178, what\u003dTimeStamp, where\u003dATLTCSFV00917;pid\u003d4652, who\u003dHCA.STHA, why\u003dHCARqstEntry\&quot;/\u003e\n                        \u003cns3:Item nm\u003d\&quot;02\&quot; val\u003d\&quot;when\u003d2017-12-20T15:23:58.179, what\u003dTimeStamp, where\u003dATLTCSFV00917;pid\u003d4652, who\u003dHCA.STHA, why\u003dHCARqstExit\&quot;/\u003e\n                    \u003c/ns3:DiagnosticCmds\u003e\n                \u003c/ns3:SysMgmt_RQ\u003e\n                \u003cns3:HCAA_Data\u003e\n                \t \u003cns3:HCAA_SessProps hstSvcID\u003d\&quot;STHA\&quot; pyldTyp\u003d\&quot;TE1G1.1\&quot; pyldLngth\u003d\&quot;187\&quot; pCIInd\u003d\&quot;false\&quot;/\u003e\n                   \n                    \u003cns3:HCAA_Payload \u003e\n                   \n                        \u003cns3:HCAATerm\u003e\u003c![CDATA[${Dynamiccomment}]]\u003e\u003c/ns3:HCAATerm\u003e\n                    \u003c/ns3:HCAA_Payload\u003e\n                \u003c/ns3:HCAA_Data\u003e\n            \u003c/ns3:HCARequest\u003e\n        \u003c/ns3:CommandRQ\u003e\n    \u003c/ns7:Body\u003e\n\u003c/ns7:Envelope\u003e&quot;,
  &quot;contentType&quot;: &quot;application/xml&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/xml</value>
      <webElementGuid>d6d52d3d-b241-4582-a03e-0f9521589921</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.2.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://hca-messaging.dvqa1.tvlport.com:8095/HcaService/CommandRQ</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.hca_ExtSessiontoken</defaultValue>
      <description></description>
      <id>635c0d93-a77d-4d0d-a8c6-3ebdfee19288</id>
      <masked>false</masked>
      <name>ExtSessionToken</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>e0d6d1f9-0430-4a38-a8eb-ce40573bf836</id>
      <masked>false</masked>
      <name>Dynamiccomment</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.PNR</defaultValue>
      <description></description>
      <id>9f9b6f95-ef29-43c9-b2b7-51995871b961</id>
      <masked>false</masked>
      <name>PNR</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.sPCC</defaultValue>
      <description></description>
      <id>60e91897-1b10-4cbb-a1ff-d10be26fe06d</id>
      <masked>false</masked>
      <name>sPCC</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.sCustomerProfileId</defaultValue>
      <description></description>
      <id>d29cd4fc-6d0e-46d4-853b-8f8fcfccc371</id>
      <masked>false</masked>
      <name>sCustomerProfileId</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
