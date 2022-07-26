<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>5_Add_Traveler_Return</name>
   <tag></tag>
   <elementGuidId>83efa806-5e33-4f4a-b4a7-81122cf1212a</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;Traveler\&quot; : {\n    \&quot;@type\&quot; : \&quot;Traveler\&quot;,\n    \&quot;birthDate\&quot; : \&quot;1999-02-22\&quot;,\n    \&quot;gender\&quot; : \&quot;Male\&quot;,\n    \&quot;PersonName\&quot; : {\n      \&quot;@type\&quot; : \&quot;PersonNameDetail\&quot;,\n      \&quot;Prefix\&quot; : \&quot;Mr\&quot;,\n      \&quot;Given\&quot; : \&quot;Px ADTOne\&quot;,\n      \&quot;Middle\&quot; : \&quot;MdNm \&quot;,\n      \&quot;Surname\&quot; : \&quot;SrNmOne\&quot;,\n      \&quot;Suffix\&quot; : \&quot;ADT\&quot;\n    },\n    \&quot;Address\&quot; : [ {\n      \&quot;@type\&quot; : \&quot;Address\&quot;,\n      \&quot;role\&quot; : \&quot;Home\&quot;,\n      \&quot;Number\&quot; : {\n        \&quot;value\&quot; : \&quot;539\&quot;\n      },\n      \&quot;Street\&quot; : \&quot;East PeakView Avenue\&quot;,\n      \&quot;City\&quot; : \&quot;Englewood\&quot;,\n      \&quot;County\&quot; : \&quot;US\&quot;,\n      \&quot;StateProv\&quot; : {\n        \&quot;name\&quot; : \&quot;Texas\&quot;,\n        \&quot;value\&quot; : \&quot;CA\&quot;\n      },\n      \&quot;PostalCode\&quot; : \&quot;917113323\&quot;\n    } ],\n    \&quot;Telephone\&quot; : [ {\n      \&quot;@type\&quot; : \&quot;Telephone\&quot;,\n      \&quot;countryAccessCode\&quot; : \&quot;1\&quot;,\n      \&quot;areaCityCode\&quot; : \&quot;909\&quot;,\n      \&quot;phoneNumber\&quot; : \&quot;212456121\&quot;,\n      \&quot;extension\&quot; : \&quot;1243\&quot;,\n      \&quot;id\&quot; : \&quot;4\&quot;,\n      \&quot;cityCode\&quot; : \&quot;ORD\&quot;,\n      \&quot;role\&quot; : \&quot;Office\&quot;\n    } ],\n    \&quot;Email\&quot; : [ {\n      \&quot;value\&quot; : \&quot;TravelerOneOne@gmail.com\&quot;\n    }, {\n      \&quot;value\&quot; : \&quot;TravelerOneTwo@gmail.com\&quot;\n    } ],\n    \&quot;CustomerLoyalty\&quot; : [ {\n      \&quot;supplier\&quot; : \&quot;DL\&quot;,\n      \&quot;value\&quot; : \&quot;DL2071983684\&quot;\n    } ],\n    \&quot;TravelDocument\&quot; : [ {\n      \&quot;@type\&quot; : \&quot;TravelDocumentDetail\&quot;,\n      \&quot;docNumber\&quot; : \&quot;245968\&quot;,\n      \&quot;docType\&quot; : \&quot;Passport\&quot;,\n      \&quot;issueDate\&quot; : \&quot;2019-12-22\&quot;,\n      \&quot;expireDate\&quot; : \&quot;2027-02-22\&quot;,\n      \&quot;issueCountry\&quot; : \&quot;IND\&quot;,\n      \&quot;birthDate\&quot; : \&quot;1999-02-22\&quot;,\n      \&quot;birthCountry\&quot; : \&quot;IND\&quot;,\n      \&quot;Gender\&quot; : \&quot;Male\&quot;,\n      \&quot;PersonName\&quot; : {\n        \&quot;@type\&quot; : \&quot;PersonName\&quot;,\n        \&quot;Given\&quot; : \&quot;Px One\&quot;,\n        \&quot;Middle\&quot; : \&quot;MdNm One\&quot;,\n        \&quot;Surname\&quot; : \&quot;SrNmOne\&quot;\n      },\n      \&quot;IssuedForGeoPoliticalArea\&quot; : {\n        \&quot;value\&quot; : \&quot;IND\&quot;\n      }\n    } ]\n  }\n}&quot;,
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
      <webElementGuid>a9162cbe-201e-4c2f-9e43-2b029d8378fe</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>22135ed3-f158-4a92-a242-2c05a6bf24a8</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept-Version</name>
      <type>Main</type>
      <value>11</value>
      <webElementGuid>cca6aa16-5847-4b1b-b902-6c0380448919</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic VW5pdmVyc2FsIEFQSS91QVBJNDM1NDgyNTAwMC0yNDk1YTRlNjpwN0EzeVV6RA==</value>
      <webElementGuid>00bc4a9d-abf7-47ce-bc2f-8e4c9acbbdfc</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>E2ETrackingID</name>
      <type>Main</type>
      <value>Samurai_Test</value>
      <webElementGuid>4fb063f1-7350-4cf2-b4c4-d86766c949fd</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>XAUTH_TRAVELPORT_ACCESSGROUP</name>
      <type>Main</type>
      <value>${sAccessGroup}</value>
      <webElementGuid>d6bfde26-03a5-4cc9-94ce-73e143f6387c</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Version</name>
      <type>Main</type>
      <value>11</value>
      <webElementGuid>3f464a61-992d-4b01-9379-3a12cd85a3ba</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>OAUTH_RESOURCEOWNERINFO</name>
      <type>Main</type>
      <value>${sAccessGroup}</value>
      <webElementGuid>1a11fd96-f363-4452-93ee-5a5ba700f7a2</webElementGuid>
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

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>