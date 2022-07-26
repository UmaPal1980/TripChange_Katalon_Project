<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>5_Add_Traveler</name>
   <tag></tag>
   <elementGuidId>089bbff1-b227-4317-a9c6-24bf5ee84e15</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;Traveler\&quot;: {\n        \&quot;passengerTypeCode\&quot;: \&quot;ADT\&quot;,\n        \&quot;PersonNameDetail\&quot;: {\n            \&quot;Prefix\&quot;: \&quot;\&quot;,\n            \&quot;Given\&quot;: \&quot;Josiah\&quot;,\n            \&quot;Middle\&quot;: \&quot;Luke\&quot;,\n            \&quot;Surname\&quot;: \&quot;Randazzo\&quot;,\n            \&quot;Suffix\&quot;: \&quot;\&quot;\n        },\n        \&quot;Telephone\&quot;: [\n            {\n                \&quot;@type\&quot;: \&quot;Telephone\&quot;,\n                \&quot;countryAccessCode\&quot;: \&quot;1\&quot;,\n                \&quot;areaCityCode\&quot;: \&quot;719\&quot;,\n                \&quot;phoneNumber\&quot;: \&quot;6401108\&quot;,\n                \&quot;extension\&quot;: \&quot;123\&quot;,\n                \&quot;id\&quot;: \&quot;1\&quot;,\n                \&quot;cityCode\&quot;: \&quot;DEN\&quot;,\n                \&quot;role\&quot;: \&quot;Mobile\&quot;\n            }\n        ],\n        \&quot;TravelDocument\&quot;: [\n            {\n                \&quot;@type\&quot;: \&quot;TravelDocumentDetail\&quot;,\n                \&quot;docNumber\&quot;: \&quot;88989889\&quot;,\n                \&quot;docType\&quot;: \&quot;Passport\&quot;,\n                \&quot;Gender\&quot;: \&quot;Male\&quot;,\n                \&quot;issueDate\&quot;: \&quot;2019-07-04\&quot;,\n                \&quot;expireDate\&quot;: \&quot;2022-12-05\&quot;,\n                \&quot;issueCountry\&quot;: \&quot;USA\&quot;,\n                \&quot;birthDate\&quot;: \&quot;1991-12-12\&quot;,\n                \&quot;birthPlace\&quot;: \&quot;USA\&quot;,\n                \&quot;personName\&quot;: {\n                    \&quot;@type\&quot;: \&quot;PersonNameDetail\&quot;,\n                    \&quot;Prefix\&quot;: \&quot;\&quot;,\n                    \&quot;Given\&quot;: \&quot;Josiah\&quot;,\n                    \&quot;Middle\&quot;: \&quot;Luke\&quot;,\n                    \&quot;Surname\&quot;: \&quot;Randazzo\&quot;,\n                    \&quot;Suffix\&quot;: \&quot;\&quot;\n                },\n                \&quot;IssuedForGeoPoliticalArea\&quot;: {\n                    \&quot;value\&quot;: \&quot;USA\&quot;\n                }\n            }\n        ],\n        \&quot;Address\&quot;: [\n            {\n                \&quot;role\&quot;: \&quot;Delivery\&quot;,\n                \&quot;Number\&quot;: \&quot;1220\&quot;,\n                \&quot;Street\&quot;: \&quot;Travers Street\&quot;,\n                \&quot;City\&quot;: \&quot;Claremont\&quot;,\n                \&quot;StateProv\&quot;: {\n                    \&quot;name\&quot;: \&quot;Texas\&quot;,\n                    \&quot;value\&quot;: \&quot;CA\&quot;\n                },\n                \&quot;Country\&quot;: {\n                    \&quot;value\&quot;: \&quot;US\&quot;\n                },\n                \&quot;PostalCode\&quot;: \&quot;917113323\&quot;\n            }\n        ],\n        \&quot;Email\&quot;: [\n            {\n                \&quot;value\&quot;: \&quot;jl_randazzo@yahoo.com\&quot;\n            }\n        ]\n    }\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>a2b42f2d-869e-4f4c-8b8c-d2f18dcc1ef6</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>e02ae90b-0664-40c3-9c9a-1b0f450b7b9b</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept-Version</name>
      <type>Main</type>
      <value>11</value>
      <webElementGuid>ebb06c54-d582-4875-bc59-c953d7878e5a</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic VW5pdmVyc2FsIEFQSS91QVBJNDM1NDgyNTAwMC0yNDk1YTRlNjpwN0EzeVV6RA==</value>
      <webElementGuid>edd3a1d4-cf11-451a-bfc5-19ccb91e4ecf</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>E2ETrackingID</name>
      <type>Main</type>
      <value>Samurai_Test</value>
      <webElementGuid>45a57ea9-87eb-4e53-9762-30d27ea3755b</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>XAUTH_TRAVELPORT_ACCESSGROUP</name>
      <type>Main</type>
      <value>${sAccessGroup}</value>
      <webElementGuid>f5f10b41-3613-40e1-bf72-1ca94e7aeec0</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Version</name>
      <type>Main</type>
      <value>11</value>
      <webElementGuid>76de7ee7-568f-4086-a02b-b004079f798b</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>OAUTH_RESOURCEOWNERINFO</name>
      <type>Main</type>
      <value>${sAccessGroup}</value>
      <webElementGuid>8fa0e181-1e8e-4a4f-be83-b312202e56c4</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.2.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${sAddTravellerURL}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.reservationId</defaultValue>
      <description></description>
      <id>d5ac00c6-add7-4ca7-a3f7-ab99684c1232</id>
      <masked>false</masked>
      <name>reservationId</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.sAccessGroup</defaultValue>
      <description></description>
      <id>71cc8681-ac05-492e-8ccf-e269eac5e368</id>
      <masked>false</masked>
      <name>sAccessGroup</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.sAddTravellerURL</defaultValue>
      <description></description>
      <id>917167ce-2e49-4acd-b4a3-141bcc35fa74</id>
      <masked>false</masked>
      <name>sAddTravellerURL</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
