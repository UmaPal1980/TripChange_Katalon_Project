<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>1_Search Request</name>
   <tag></tag>
   <elementGuidId>9506bf18-1ede-4427-bf67-cea12acc5ad3</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;CatalogOfferingsQueryRequest\&quot;: {\n    \&quot;CatalogOfferingsRequest\&quot;: [\n      {\n        \&quot;@type\&quot;: \&quot;CatalogOfferingsRequestAir\&quot;,\n        \&quot;maxNumberOfOffersToReturn\&quot;: 200,\n        \&quot;offersPerPage\&quot;: 200,\n        \&quot;contentSourceList\&quot;: [\n          \&quot;GDS\&quot;\n        ],\n        \&quot;returnBrandedFaresInd\&quot;: true,\n        \&quot;detailViewInd\&quot;: true,\n        \&quot;upsellInd\&quot;: true,\n        \&quot;PassengerCriteria\&quot;: [\n          {\n            \&quot;@type\&quot;: \&quot;PassengerCriteria\&quot;,\n            \&quot;number\&quot;: \&quot;${sPassengerCount}\&quot;,\n            \&quot;passengerTypeCode\&quot;: \&quot;${sPTC}\&quot;\n          }\n        ],\n        \&quot;SearchCriteriaFlight\&quot;: [\n          {\n            \&quot;departureDate\&quot;: \&quot;${sDepDate}\&quot;,\n            \&quot;From\&quot;: {\n              \&quot;value\&quot;: \&quot;${sFrom}\&quot;\n            },\n            \&quot;To\&quot;: {\n              \&quot;value\&quot;: \&quot;${sTo}\&quot;\n            }\n          }\n        ],\n        \&quot;SearchModifiersAir\&quot;: {\n          \&quot;excludeGround\&quot;: \&quot;Train\&quot;,\n          \&quot;CarrierPreference\&quot;: [\n            {\n              \&quot;preferenceType\&quot;: \&quot;Preferred\&quot;,\n              \&quot;carriers\&quot;: [\n                \&quot;${sCarrier}\&quot;\n              ]\n            }\n          ],\n          \&quot;CabinPreference\&quot;: [\n            {\n              \&quot;preferenceType\&quot;: \&quot;Preferred\&quot;,\n              \&quot;cabins\&quot;: [\n                \&quot;Economy\&quot;\n              ]\n            }\n          ],\n          \&quot;ConnectionPreferences\&quot;: [\n            {\n              \&quot;@type\&quot;: \&quot;ConnectionPreferencesAir\&quot;,\n              \&quot;FlightType\&quot;: {\n                \&quot;connectionType\&quot;: \&quot;DoubleConnection\&quot;,\n                \&quot;excludeInterlineConnectionsInd\&quot;: true\n              }\n            }\n          ]\n        },\n        \&quot;PricingModifiersAir\&quot;: {\n          \&quot;@type\&quot;: \&quot;PricingModifiersAir\&quot;,\n          \&quot;currencyCode\&quot;: \&quot;${sCurrencyCode}\&quot;,\n          \&quot;FareSelection\&quot;: {\n            \&quot;@type\&quot;: \&quot;FareSelectionDetail\&quot;,\n            \&quot;prohibitMaxStayFaresInd\&quot;: true,\n            \&quot;fareType\&quot;: \&quot;PublicFaresOnly\&quot;\n          }\n        }\n      }\n    ]\n  }\n}&quot;,
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
      <webElementGuid>9b28b24e-e977-460c-9976-b7ded66104e2</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>e911b33a-8c24-47aa-8716-2991e5f4df7c</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept-Version</name>
      <type>Main</type>
      <value>11</value>
      <webElementGuid>1eea6dcb-ec2f-41e8-929e-87d24080d72b</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>E2ETrackingID</name>
      <type>Main</type>
      <value>Samurai_Test</value>
      <webElementGuid>d713ad99-4cd2-4ffc-9755-e35d22f90c51</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>XAUTH_TRAVELPORT_ACCESSGROUP</name>
      <type>Main</type>
      <value>${sAccessGroup}</value>
      <webElementGuid>6206921a-b66e-48bf-b2ff-bb9ab08425fb</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Version</name>
      <type>Main</type>
      <value>11</value>
      <webElementGuid>5d1cab6d-1e9e-4396-b3a9-2b1be5f0b0bd</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>VW5pdmVyc2FsIEFQSS91QVBJNDM1NDgyNTAwMC0yNDk1YTRlNjpwN0EzeVV6RA==</value>
      <webElementGuid>1d7fb93a-8a26-49a0-a048-c47b6ab3fb43</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>OAUTH_RESOURCEOWNERINFO</name>
      <type>Main</type>
      <value>${sAccessGroup}</value>
      <webElementGuid>71bc2f7f-8128-433f-8b18-722e13950e9e</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>OAuth_ClientID</name>
      <type>Main</type>
      <value>qa-sXZ3EygiKuovA7RgGjDbZ1PIRimxetbaAGczeGnW</value>
      <webElementGuid>4f3f73bc-41a4-48f8-863f-8183730e7e95</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.2.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${sSearchURL}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.sFrom</defaultValue>
      <description></description>
      <id>4fb9e7e1-4867-454f-9018-dcf8f955ce12</id>
      <masked>false</masked>
      <name>sFrom</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.sTo</defaultValue>
      <description></description>
      <id>e0ff7f0a-bc8e-477d-9238-473804e41d2d</id>
      <masked>false</masked>
      <name>sTo</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.sCarrier</defaultValue>
      <description></description>
      <id>6864a548-3f2a-407c-a373-5e3874ed6b64</id>
      <masked>false</masked>
      <name>sCarrier</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.sCurrencyCode</defaultValue>
      <description></description>
      <id>85306f97-c8de-46dc-9f7d-3daa4d75611b</id>
      <masked>false</masked>
      <name>sCurrencyCode</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.sPTC</defaultValue>
      <description></description>
      <id>b3a8e19f-731d-452b-bb19-c3d4062b3e1b</id>
      <masked>false</masked>
      <name>sPTC</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.sPassengerCount</defaultValue>
      <description></description>
      <id>f046e3e3-48bc-4d7a-90ba-6cd8d9124b87</id>
      <masked>false</masked>
      <name>sPassengerCount</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.sAccessGroup</defaultValue>
      <description></description>
      <id>baa27226-b331-4888-be7d-ac9d1831a704</id>
      <masked>false</masked>
      <name>sAccessGroup</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.sDepDate</defaultValue>
      <description></description>
      <id>4af81fb2-c63a-4e1d-80da-555647989547</id>
      <masked>false</masked>
      <name>sDepDate</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.sRetDate</defaultValue>
      <description></description>
      <id>90ac831c-f64e-4888-a420-97b0eb6c4dca</id>
      <masked>false</masked>
      <name>sRetDate</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.sSearchURL</defaultValue>
      <description></description>
      <id>d941774b-2998-4b5c-a57a-b9ffc9656952</id>
      <masked>false</masked>
      <name>sSearchURL</name>
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


def jsonSlurper = new JsonSlurper()

//def jsonResponseText = jsonSlurper.parseText(response.getResponseText())

//def jsonResponse = jsonSlurper.parseText(response.getResponseBodyContent())

//assertThat(response.getStatusCode()).isEqualTo(200)

//GlobalVariable.sSearchToken = jsonResponse.CatalogOfferingsResponse.CatalogOfferings.Identifier.value

//println('Value of Search Identifier Token is ' + GlobalVariable.sSearchToken)

</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
