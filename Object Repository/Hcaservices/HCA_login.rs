<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>HCA_login</name>
   <tag></tag>
   <elementGuidId>7c561cf0-e41d-46e1-8dd7-09b64bcbc4cd</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;\u003cns7:Envelope xmlns:ns2\u003d\&quot;http://www.travelport.com/otm/builtins/tp/msgcommon/v1_0_0\&quot; xmlns:ns3\u003d\&quot;http://www.travelport.com/sandbox/hca/schemas/hca/v1\&quot; xmlns:ns4\u003d\&quot;http://www.opentravel.org/OTM/Common/v0\&quot; xmlns:ns5\u003d\&quot;http://www.travelport.com/schemas/hca/v0\&quot; xmlns:ns6\u003d\&quot;http://www.travelport.com/otm/builtins/tp/message/v1_0_0\&quot; xmlns:ns7\u003d\&quot;http://schemas.xmlsoap.org/soap/envelope/\&quot;\u003e\n    \u003cns7:Header\u003e\n        \u003cns5:SessionContextHCA\u003e\n            \u003cns5:E2ETrackingID\u003ebdf6e1fe-efcf-4c71-9690-d2ecd963aa9f\u003c/ns5:E2ETrackingID\u003e\n            \u003cns5:ChildTrackingID\u003e12528673-806a-4aba-afb2-e0e3f6ea89b6\u003c/ns5:ChildTrackingID\u003e\n            \u003cns5:OriginApplication\u003eUAPI\u003c/ns5:OriginApplication\u003e\n            \u003cns5:CustomerProfileID\u003e${sCustomerProfileId}\u003c/ns5:CustomerProfileID\u003e\n            \u003cns5:SessionDisposition\u003eFIRST\u003c/ns5:SessionDisposition\u003e\n            \u003c!--\u003cns5:SessionDisposition\u003eMIDDLE\u003c/ns5:SessionDisposition\u003e\n            \u003cns5:ExternalSessionToken\u003e71E1777D-49E8-4F69-5063-51E01907E0D3\u003c/ns5:ExternalSessionToken\u003e--\u003e\n            \u003cns5:PseudoCityCode\u003e${sPCC}\u003c/ns5:PseudoCityCode\u003e\n             \u003cns5:NameValuePair name\u003d\&quot;TRANSPORT\&quot;\u003eJCPMGR:NOLOG\u003c/ns5:NameValuePair\u003e\n            \u003c!--\u003cns5:TPFSYSID\u003eE735615\u003c/ns5:TPFSYSID\u003e--\u003e\n        \u003c/ns5:SessionContextHCA\u003e\n    \u003c/ns7:Header\u003e\n    \u003cns7:Body\u003e\n        \u003cns3:CommandRQ ReturnSoapFault\u003d\&quot;false\&quot;\u003e\n            \u003cns3:HCARequest retryCnt\u003d\&quot;0\&quot;\u003e\n                \u003cns3:SysMgmt_RQ\u003e\n                    \u003cns3:HCASysMgmtSessProp_SysMgmtCmd mep\u003d\&quot;RR\&quot;\u003e\n                        \u003cns3:Endpoint\u003e1G\u003c/ns3:Endpoint\u003e\n                        \u003c!--\u003cns3:GTIDLEIDOvrd\u003eFE425C\u003c/ns3:GTIDLEIDOvrd\u003e--\u003e\n                        \u003cns3:DiagCmdCds timeStamp\u003d\&quot;EntryAndExit\&quot;\u003e\n                            \u003cns3:SysRsrcsCnsmd\u003eNodes\u003dTPF; ecbExistTime; CPUMills; DiskRd; DiskWrt; PrgCalls\u003c/ns3:SysRsrcsCnsmd\u003e\n                            \u003cns3:SysOpsStats\u003eNodes\u003dTPF;IStrm\u003dCurrent\u003c/ns3:SysOpsStats\u003e\n                        \u003c/ns3:DiagCmdCds\u003e\n                        \u003cns3:Elog\u003eNone\u003c/ns3:Elog\u003e\n                    \u003c/ns3:HCASysMgmtSessProp_SysMgmtCmd\u003e\n                    \u003cns3:DiagnosticCmds\u003e\n                        \u003cns3:Item nm\u003d\&quot;01\&quot; val\u003d\&quot;when\u003d2017-12-20T15:23:58.178, what\u003dTimeStamp, where\u003dATLTCSFV00917;pid\u003d4652, who\u003dHCA.STHA, why\u003dHCARqstEntry\&quot;/\u003e\n                        \u003cns3:Item nm\u003d\&quot;02\&quot; val\u003d\&quot;when\u003d2017-12-20T15:23:58.179, what\u003dTimeStamp, where\u003dATLTCSFV00917;pid\u003d4652, who\u003dHCA.STHA, why\u003dHCARqstExit\&quot;/\u003e\n                    \u003c/ns3:DiagnosticCmds\u003e\n                \u003c/ns3:SysMgmt_RQ\u003e\n                \u003cns3:HCAA_Data\u003e\n                    \u003cns3:HCAA_SessProps hstSvcID\u003d\&quot;STHA\&quot; pyldTyp\u003d\&quot;KLR1.1\&quot; pyldLngth\u003d\&quot;120\&quot; pCIInd\u003d\&quot;false\&quot;/\u003e\n                    \u003c!--\u003cns3:HCAA_SessProps hstSvcID\u003d\&quot;STHA\&quot; pyldTyp\u003d\&quot;DIR1.1\&quot; pyldLngth\u003d\&quot;187\&quot; pCIInd\u003d\&quot;false\&quot;/\u003e--\u003e\n                    \u003cns3:HCAA_Payload\u003e\n                       \u003c!--\u003cns3:HCAATerm\u003e\u003c![CDATA[ZRLMT 034489 BP]]\u003e\u003c/ns3:HCAATerm\u003e--\u003e\n                       \u003c!--\u003cns3:HCAADIR\u003e                  \n                       \u003c![CDATA[DIR0DER0000001470011P1P0001P1PQL00000001010101S00000000000000000000001$0000000000030013ERQ254*30014ETRA240*30023ETS227F131693ETN2130167205043637000]]\u003e\u003c/ns3:HCAADIR\u003e--\u003e\n                        \u003c!--\u003cns3:HCAAKLR\u003e\u003c![CDATA[0034ADSR000F00011000000106000100000072AAGA000F000501 20180510LAX JFK 0001D00012359 BBYN YYYYNYYN 0034ADSR000F00011400001538001300000211GQQH003F0012SBB $PDIRBL 1G 6G2D201804231255 201804231255LON AG99999999999996G2D9999999 01 006 Y GBLONN3N GB YGBLON YY 1G Y YNNN NNN0070GQPA001F0005010101ADT24 0058GQPF000F000200NNNNYYNN6G2D9999999N N 0052GQFR010F0001AAFIAAONERORGFGQGFSRGFQAGFISGFBRGFPI0044GQTO000F0004 Y LWS TPD 0081AAOO000F00011G6G2DLON LON 99999992GBAG T LAX 20180510 0108GQQM000F0013LON LON 6G2D Y0692GQAU169F0001AA AB AC AF AI AM AR AS AT AV AY AZ BA BE BI BP BR BS BT BW CA CF CO CU CX CY CZ DE DL DT DU DY EI EK ET EY FB FI FJ FV GA GF GR HM HR HX IB IC IG IT IY JE JJ JK JL JP JZ KA KC KL KM KQ KU LA LB LH LO LV LW LX LY MA MD ME MH MK MS MU MY NH NX NZ OA OC OK OL OM OS OU OV OZ PC PG PS PW PX QF QR QV RC RE RJ RO RQ SA SB SK SN SQ SU SW SY TG TK TM TN TP TS TU UA UL UN UO UP US UT UX VG VH VN VQ VS VT VV WM WY YM YO YP ZH ZI A3 B2 F7 F9 I5 I9 J2 J9 S3 S4 S7 T7 W2 W3 Z8 2I 2J 4Q 4S 5L 6H 7C 9B 9F 9U 9W P0 1G 0026GQTF001F0001000000000 0027GQA1000F0001LON 6G2DO0031GQBU000F000303000NYNY0100NN0036FLTR004F00011001AAFIAAFJAAB1AAON0068FLTR012F00011425ERORGFGQGFSRGFQAGFISGFBRGFPIGFRIGFPXGFMMGFEEGFGT]]\u003e\u003c/ns3:HCAAKLR\u003e--\u003e\n                        \u003cns3:HCAAKLR\u003e\u003c![CDATA[PQQ010001TSV01SIGNONLDIN015650010000041K000000010041GSON000F0001ZGWS                  ]]\u003e\u003c/ns3:HCAAKLR\u003e\n                        \u003c!--\u003cns3:HCAATerm\u003e\u003c![CDATA[*MJ0GP6]]\u003e\u003c/ns3:HCAATerm\u003e--\u003e\n                    \u003c/ns3:HCAA_Payload\u003e\n                \u003c/ns3:HCAA_Data\u003e\n            \u003c/ns3:HCARequest\u003e\n        \u003c/ns3:CommandRQ\u003e\n    \u003c/ns7:Body\u003e\n\u003c/ns7:Envelope\u003e&quot;,
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
      <webElementGuid>e81377fa-0056-45d1-b9cb-bccff53b9594</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.2.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${sHCA_URL}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.sPCC</defaultValue>
      <description></description>
      <id>951b1bba-e976-4a06-8e37-bf31b17062c4</id>
      <masked>false</masked>
      <name>sPCC</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.sCustomerProfileId</defaultValue>
      <description></description>
      <id>d53186a6-c072-4ae8-b696-44185f80bc1d</id>
      <masked>false</masked>
      <name>sCustomerProfileId</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.sHCA_URL</defaultValue>
      <description></description>
      <id>7406575f-9213-4280-bfc8-3fe9fe744424</id>
      <masked>false</masked>
      <name>sHCA_URL</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import groovy.util.XmlSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

def xmlSlurpur=new XmlSlurper()

def xmlResponse=xmlSlurpur.parseText(response.responseBodyContent)

GlobalVariable.hca_ExtSessiontoken=xmlResponse.Header.SessionContextHCA.ExternalSessionToken

println(GlobalVariable.hca_ExtSessiontoken)
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
